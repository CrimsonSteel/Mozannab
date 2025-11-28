use std::sync::mpsc::{Sender, Receiver};
use crate::events::AppEvent;
use crate::commands::Command as CalibrationCommand;

// long-running calibration...
pub fn opencv_posture_calibrate(tx: Sender<AppEvent>, rx: &Receiver<CalibrationCommand>) {

    let _ = tx.send(AppEvent::CalibrationStarted);
    
    // Good pose calibration phase
    for i in 0..=10 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        let progress = (i as f32) / 20.0;
        let _ = tx.send(AppEvent::CalibrationProgress(progress));
    }

    let _ = tx.send(AppEvent::CalibrationGoodPoseFinished);


    // Wait for proceeding signal
    dbg!("Waiting for user to proceed from good pose...");
    while let Ok(cmd) = rx.recv() {
        match cmd {
            CalibrationCommand::CalibrationCancel => {
                let _ = tx.send(AppEvent::CalibrationError("Calibration cancelled by user.".to_string()));
                return;
            }
            CalibrationCommand::CalibrationProceed => {
                break;
            }
        }
    }

    for i in 0..=10 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        let progress = 0.5 + (i as f32) / 20.0;
        let _ = tx.send(AppEvent::CalibrationProgress(progress));
    }

    let _ = tx.send(AppEvent::CalibrationBadPoseFinished);

    // simulating finalization
    std::thread::sleep(std::time::Duration::from_millis(2500));

    
    let _ = tx.send(AppEvent::CalibrationCompleted);

}