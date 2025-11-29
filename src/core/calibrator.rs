use std::sync::mpsc::{Sender, Receiver};
use crate::events::AppEvent;
use crate::commands::Command as CalibrationCommand;


fn wait_for_calibration_proceed(rx: &Receiver<CalibrationCommand>) -> bool {
    while let Ok(cmd) = rx.recv() {
        match cmd {
            CalibrationCommand::CalibrationCancel => {
                return false;
            }
            CalibrationCommand::CalibrationProceed => {
                return true;
            }
        }
    }
    false
}


// long-running calibration...
pub fn opencv_posture_calibrate(tx: Sender<AppEvent>, rx: &Receiver<CalibrationCommand>, selected_camera: i32) {

    let _ = tx.send(AppEvent::CalibrationStarted);
    
    let pose_proc_configs = super::PostureProcConfig::default();

    // opening the camera

    let Ok(mut cam) = super::videocapture::open_camera(selected_camera, None) else {
        let _ = tx.send(AppEvent::CalibrationError("Failed to open camera for calibration.".to_string()));
        return;
    };

    // collecting good pose samples

    let mut good_frames = Vec::new();

    for _ in 0..pose_proc_configs.samples_needed {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let Ok(frame) = super::videocapture::read_frame(&mut cam, true) else {
            let _ = tx.send(AppEvent::CalibrationError("Failed to read frame from camera.".to_string()));
            let _ = super::videocapture::close_camera(&mut cam);
            return;
        };
        good_frames.push(frame);

        let progress = (good_frames.len() as f32) / (pose_proc_configs.samples_needed as f32);
        let _ = tx.send(AppEvent::CalibrationProgress(progress));
    }

    let _ = tx.send(AppEvent::CalibrationGoodPoseFinished);


    // Wait for proceed signal
    let proceed = wait_for_calibration_proceed(rx);
    if !proceed {
        let _ = tx.send(AppEvent::CalibrationError("Calibration cancelled by user.".to_string()));
        let _ = super::videocapture::close_camera(&mut cam);
        return;
    }

    // collecting bad pose samples

    let mut bad_frames = Vec::new();

    for _ in 0..pose_proc_configs.samples_needed {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let Ok(frame) = super::videocapture::read_frame(&mut cam, true) else {
            let _ = tx.send(AppEvent::CalibrationError("Failed to read frame from camera.".to_string()));
            let _ = super::videocapture::close_camera(&mut cam);
            return;
        };
        bad_frames.push(frame);

        let progress = (bad_frames.len() as f32) / (pose_proc_configs.samples_needed as f32);
        let _ = tx.send(AppEvent::CalibrationProgress(progress));
    }

    let _ = tx.send(AppEvent::CalibrationBadPoseFinished);


    // Batched face detections for both good and bad frames

    let Ok(good_detections) = super::model::batch_inference(&good_frames) else {
        let _ = tx.send(AppEvent::CalibrationError("Failed to perform face detection during calibration.".to_string()));
        let _ = super::videocapture::close_camera(&mut cam);
        return;
    };

    let Ok(bad_detections) = super::model::batch_inference(&bad_frames) else {
        let _ = tx.send(AppEvent::CalibrationError("Failed to perform face detection during calibration.".to_string()));
        let _ = super::videocapture::close_camera(&mut cam);
        return;
    };

    // TODO: Handle cases where few faces are detected
    let good_areas = good_detections.iter().map(|d| d.area).collect::<Vec<f64>>();
    let bad_areas = bad_detections.iter().map(|d| d.area).collect::<Vec<f64>>();

    // Using MAD-filter to reject outliers
    let good_median_area = super::utils::mad_filter_median(&good_areas, pose_proc_configs.mad_filter_k);
    let bad_median_area = super::utils::mad_filter_median(&bad_areas, pose_proc_configs.mad_filter_k);
    let midpoint = (good_median_area + bad_median_area) / 2.0;

    let calibration_result = super::CalibrationResult {
        posture_good_baseline: good_median_area,
        posture_bad_baseline: bad_median_area,
        posture_midpoint: midpoint,
    };

    let _ = tx.send(AppEvent::CalibrationCompleted(calibration_result));

}