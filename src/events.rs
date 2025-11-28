use std::sync::mpsc::{self, Receiver, Sender};

pub enum AppEvent {
    CalibrationGoodPoseFinished,
    CalibrationBadPoseFinished,

    CalibrationStarted,
    CalibrationProgress(f32),   
    CalibrationCompleted,
    CalibrationError(String),
}



pub fn create_events_channel() -> (Sender<AppEvent>, Receiver<AppEvent>) {
    mpsc::channel()
}
