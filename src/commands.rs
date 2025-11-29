use std::sync::mpsc::{self, Receiver, Sender};

pub enum Command {
    CalibrationProceed,
    CalibrationCancel,

    TrackStop,
}

pub fn create_cmd_channel() -> (Sender<Command>, Receiver<Command>) {
    mpsc::channel()
}
