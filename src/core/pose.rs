use std::sync::mpsc::{Sender, Receiver};
use crate::events::AppEvent;
use crate::commands::Command as CalibrationCommand;

fn is_stop_requested(rx: &Receiver<CalibrationCommand>) -> bool {
    let Some(cmd) = rx.try_recv().ok() else {
        return false;
    };

    match cmd {
        CalibrationCommand::TrackStop => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn opencv_posture_loop(tx: Sender<AppEvent>, rx: &Receiver<CalibrationCommand>, 
    selected_camera: i32, calibration_result: super::CalibrationResult) {

    let configs = super::PostureProcConfig::default();

    let mut last_check_time = std::time::Instant::now();
    
    tx.send(AppEvent::PostureAssessmentStarted).ok();

    loop {
        if is_stop_requested(rx) {
            break;
        }

        let elapsed = last_check_time.elapsed();
        let rand_ms = rand::random::<u64>() % configs.check_interval_jitter_ms;
        if elapsed.as_millis() <= (configs.check_interval_ms + rand_ms) as u128 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            continue;
        }

        // Perform posture assessment step 
        

        // Reset timer
        last_check_time = std::time::Instant::now();
        
    }

    tx.send(AppEvent::PostureAssessmentCompleted).ok();
}