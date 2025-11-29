use crate::commands::Command as CalibrationCommand;
use crate::events::AppEvent;
use std::sync::mpsc::{Receiver, Sender};

fn wait_for_calibration_proceed(rx: &Receiver<CalibrationCommand>) -> bool {
    while let Ok(cmd) = rx.recv() {
        match cmd {
            CalibrationCommand::CalibrationCancel => {
                return false;
            }
            CalibrationCommand::CalibrationProceed => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
    false
}

// long-running calibration...
pub fn opencv_posture_calibrate(
    tx: Sender<AppEvent>,
    rx: &Receiver<CalibrationCommand>,
    selected_camera: i32,
) {
    log::info!("Calibration worker starting for camera {selected_camera}");
    let _ = tx.send(AppEvent::CalibrationStarted);

    let pose_proc_configs = super::PostureProcConfig::default();

    // opening the camera

    let Ok(mut cam) = super::videocapture::open_camera(selected_camera, None) else {
        log::error!("Calibration: failed to open camera {selected_camera}");
        let _ = tx.send(AppEvent::CalibrationError(
            "Failed to open camera for calibration.".to_string(),
        ));
        return;
    };
    log::debug!("Calibration: camera {selected_camera} opened");

    // collecting good pose samples

    let mut good_frames = Vec::new();

    for _ in 0..pose_proc_configs.calibration_samples_needed {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let Ok(frame) = super::videocapture::read_frame(&mut cam, true) else {
            let _ = tx.send(AppEvent::CalibrationError(
                "Failed to read frame from camera.".to_string(),
            ));
            let _ = super::videocapture::close_camera(&mut cam);
            return;
        };
        good_frames.push(frame);

        let progress =
            (good_frames.len() as f32) / (pose_proc_configs.calibration_samples_needed as f32);

        log::debug!(
            "Calibration: collected good sample {}/{}",
            good_frames.len(),
            pose_proc_configs.calibration_samples_needed
        );
        let _ = tx.send(AppEvent::CalibrationProgress(progress));
    }

    log::info!("Calibration: good-pose sampling finished");
    let _ = tx.send(AppEvent::CalibrationGoodPoseFinished);

    // Wait for proceed signal
    let proceed = wait_for_calibration_proceed(rx);
    if !proceed {
        log::info!("Calibration cancelled by user before bad-pose sampling");
        let _ = tx.send(AppEvent::CalibrationError(
            "Calibration cancelled by user.".to_string(),
        ));
        let _ = super::videocapture::close_camera(&mut cam);
        return;
    }

    // collecting bad pose samples

    let mut bad_frames = Vec::new();

    for _ in 0..pose_proc_configs.calibration_samples_needed {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let Ok(frame) = super::videocapture::read_frame(&mut cam, true) else {
            let _ = tx.send(AppEvent::CalibrationError(
                "Failed to read frame from camera.".to_string(),
            ));
            let _ = super::videocapture::close_camera(&mut cam);
            return;
        };
        bad_frames.push(frame);

        let progress =
            (bad_frames.len() as f32) / (pose_proc_configs.calibration_samples_needed as f32);
        log::debug!(
            "Calibration: collected bad sample {}/{}",
            bad_frames.len(),
            pose_proc_configs.calibration_samples_needed
        );
        let _ = tx.send(AppEvent::CalibrationProgress(progress));
    }

    let _ = tx.send(AppEvent::CalibrationBadPoseFinished);

    // Batched face detections for both good and bad frames

    let Ok(good_detections) = super::model::batch_inference(&good_frames) else {
        log::error!("Calibration: face detection failed on good frames");
        let _ = tx.send(AppEvent::CalibrationError(
            "Failed to perform face detection during calibration.".to_string(),
        ));
        let _ = super::videocapture::close_camera(&mut cam);
        return;
    };

    let Ok(bad_detections) = super::model::batch_inference(&bad_frames) else {
        log::error!("Calibration: face detection failed on bad frames");
        let _ = tx.send(AppEvent::CalibrationError(
            "Failed to perform face detection during calibration.".to_string(),
        ));
        let _ = super::videocapture::close_camera(&mut cam);
        return;
    };

    // TODO: Handle cases where few or no faces were detected

    let good_areas = good_detections
        .iter()
        .filter_map(|d| {
            if d.is_some() {
                Some(d.as_ref().unwrap().area)
            } else {
                None
            }
        })
        .collect::<Vec<f64>>();

    let bad_areas = bad_detections
        .iter()
        .filter_map(|d| {
            if d.is_some() {
                Some(d.as_ref().unwrap().area)
            } else {
                None
            }
        })
        .collect::<Vec<f64>>();

    // Using MAD-filter to reject outliers
    let good_median_area =
        super::utils::mad_filter_median(&good_areas, pose_proc_configs.mad_filter_k);
    let bad_median_area =
        super::utils::mad_filter_median(&bad_areas, pose_proc_configs.mad_filter_k);
    let midpoint = (good_median_area + bad_median_area) / 2.0;

    let calibration_result = super::CalibrationResult {
        posture_good_baseline: good_median_area,
        posture_bad_baseline: bad_median_area,
        posture_midpoint: midpoint,
    };

    /*dbg!("Calibration Result: Good Baseline: {}, Bad Baseline: {}, Midpoint: {}",
        calibration_result.posture_good_baseline,
        calibration_result.posture_bad_baseline,
        calibration_result.posture_midpoint
    );*/

    log::info!("Calibration completed successfully");
    let _ = tx.send(AppEvent::CalibrationCompleted(calibration_result));
}
