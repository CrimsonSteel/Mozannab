use crate::commands::Command as CalibrationCommand;
use crate::events::AppEvent;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone, Copy)]
pub struct PostureAssessment {
    pub raw_score: f64,  // Continuous score (negative = good, positive = bad)
    pub confidence: f64, // How confident we are (0.0-1.0)
    pub classification: PostureClass,
    pub severity: f64, // How bad (0.0-1.0)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostureClass {
    Good,
    Warning,
    Bad,
    Unknown,
}

pub fn assess_posture(
    posture_results: &[Option<super::FaceData>],
    calibration: &super::CalibrationResult,
    config: &super::PostureProcConfig,
) -> PostureAssessment {
    // TODO: check if midpoint shifting twoards good baseline (lesser room for user to move) can give better results
    let motion_range = (calibration.posture_good_baseline - calibration.posture_bad_baseline).abs();
    let midpoint = calibration.posture_midpoint;

    let mut valid_scores = Vec::new();

    // unused, left for possible future use
    let mut missing_count = 0;

    for detection in posture_results {
        match detection {
            Some(face) => {
                let face_diff = face.area - midpoint;
                let raw_score = face_diff / motion_range;

                // Weighted by confidence
                let weighted_score = raw_score * (face.conf as f64);
                valid_scores.push(weighted_score);
            }
            None => {
                missing_count += 1;
            }
        }
    }

    if valid_scores.is_empty() {
        return PostureAssessment {
            raw_score: 0.0,
            confidence: 0.0,
            classification: PostureClass::Unknown,
            severity: 0.0,
        };
    }

    // average score
    let avg_score: f64 = valid_scores.iter().sum::<f64>() / (valid_scores.len() as f64);

    // TODO: revise confidence calculation, but makes sense for now
    let detection_rate = (valid_scores.len() as f64) / (posture_results.len() as f64);
    let confidence = detection_rate.max(0.3); // Minimum 0.3 confidence

    // how many bad vs how manu good
    let good_count = valid_scores
        .iter()
        .filter(|&&s| s < config.score_good_threshold)
        .count();
    let bad_count = valid_scores
        .iter()
        .filter(|&&s| s > config.score_bad_threshold)
        .count();

    let good_ratio = (good_count as f64) / (valid_scores.len() as f64);
    let bad_ratio = (bad_count as f64) / (valid_scores.len() as f64);

    // Classification
    let classification = if bad_ratio >= config.history_bad_threshold {
        PostureClass::Bad
    } else if good_ratio >= 0.6 {
        PostureClass::Good
    } else {
        PostureClass::Warning
    };

    // How far inside the bad zone (0.0 = at threshold, 1.0 = extreme)
    let severity = if avg_score > config.score_bad_threshold {
        ((avg_score - config.score_bad_threshold) / (1.0 - config.score_bad_threshold)).min(1.0)
    } else {
        0.0
    };

    PostureAssessment {
        raw_score: avg_score,
        confidence,
        classification,
        severity,
    }
}

fn is_stop_requested(rx: &Receiver<CalibrationCommand>) -> bool {
    while let Ok(cmd) = rx.try_recv() {
        match cmd {
            CalibrationCommand::TrackStop => {
                return true;
            }
            _ => {
                // ignore other commands here
            }
        }
    }

    false
}

pub fn opencv_posture_loop(
    tx: Sender<AppEvent>,
    rx: &Receiver<CalibrationCommand>,
    selected_camera: i32,
    calibration_result: super::CalibrationResult,
) {
    let pose_proc_configs = super::PostureProcConfig::default();

    let mut last_check_time = std::time::Instant::now();

    log::info!("Posture worker starting for camera {selected_camera}");

    while let Ok(cmd) = rx.try_recv() {
        log::debug!("Draining pre-start command: {cmd:?}");
    }

    tx.send(AppEvent::PostureAssessmentStarted).ok();
    log::info!("Posture assessment started");

    loop {
        if is_stop_requested(rx) {
            log::info!("Posture worker stop requested");
            break;
        }

        let elapsed = last_check_time.elapsed();
        let rand_ms = rand::random::<u64>() % pose_proc_configs.check_interval_jitter_ms;
        if elapsed.as_millis() <= (pose_proc_configs.check_interval_ms + rand_ms) as u128 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            continue;
        }

        // Perform posture assessment step
        let Ok(mut cam) = super::videocapture::open_camera(selected_camera, None) else {
            log::error!("Posture: failed to open camera {selected_camera}");
            let _ = tx.send(AppEvent::PostureAssessmentError(
                "Failed to open camera for posture assessment.".to_string(),
            ));
            return;
        };
        log::debug!("Posture: camera {selected_camera} opened");

        let mut frames: Vec<opencv::core::Mat> =
            Vec::with_capacity(pose_proc_configs.history_size);

        for _ in 0..pose_proc_configs.history_size {
            let Ok(frame) = super::videocapture::read_frame(&mut cam, true) else {
                log::error!(
                    "Posture: failed to read frame from camera {selected_camera}"
                );
                let _ = tx.send(AppEvent::PostureAssessmentError(
                    "Failed to read frame from camera.".to_string(),
                ));
                let _ = super::videocapture::close_camera(&mut cam);
                return;
            };
            frames.push(frame);
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        // Process frames for posture assessment
        let Ok(posture_results) = super::model::batch_inference(&frames) else {
            let _ = tx.send(AppEvent::PostureAssessmentError(
                "Failed to perform posture assessment.".to_string(),
            ));
            let _ = super::videocapture::close_camera(&mut cam);
            return;
        };

        let assessment = assess_posture(&posture_results, &calibration_result, &pose_proc_configs);
        log::debug!(
            "Posture Assessment: score={:.3}, confidence={:.3}, class={:?}, severity={:.3}",
            assessment.raw_score,
            assessment.confidence,
            assessment.classification,
            assessment.severity
        );

        match assessment.classification {
            PostureClass::Bad | PostureClass::Warning => {
                let severity_msg = if assessment.severity > 0.7 {
                    "Severe bad posture detected!"
                } else if assessment.severity > 0.4 {
                    "Bad posture detected."
                } else {
                    "Slight bad posture."
                };

                // Avoid spamming info for small issues
                if assessment.severity > 0.7 {
                    log::warn!("{} (severity={:.2})", severity_msg, assessment.severity);
                } else if assessment.severity > 0.4 {
                    log::info!("{} (severity={:.2})", severity_msg, assessment.severity);
                } else {
                    log::debug!("{} (severity={:.2})", severity_msg, assessment.severity);
                }

                let _ = tx.send(AppEvent::PostureAssessmentWarning((
                    PostureClass::Bad,
                    assessment.severity,
                    severity_msg.to_string(),
                )));
            }
            _ => {}
        }

        let _ = tx.send(AppEvent::PostureAssessmentScore(assessment.raw_score));

        let _ = super::videocapture::close_camera(&mut cam);
        log::debug!("Posture: camera {selected_camera} closed");

        last_check_time = std::time::Instant::now();
    }

    log::info!(
        "Posture assessment worker completed for camera {selected_camera}"
    );
    tx.send(AppEvent::PostureAssessmentCompleted).ok();
}
