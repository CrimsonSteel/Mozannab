use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PoseErrors {
    CameraOpenError,
    FrameReadError,
    FaceDetectionError,
    PostureAssessmentError,
}

pub enum AppEvent {
    CalibrationGoodPoseFinished,
    CalibrationBadPoseFinished,

    CalibrationStarted,
    CalibrationProgress(f32),
    CalibrationCompleted(super::core::CalibrationResult),
    CalibrationError(String),

    PostureAssessmentStarted,
    PostureAssessmentCompleted,
    PostureAssessmentError(PoseErrors, String),
    PostureAssessmentWarning((crate::core::pose::PostureClass, f64, String)), // class, severity, and message
    PostureAssessmentScore(f64),
}

pub fn create_events_channel() -> (Sender<AppEvent>, Receiver<AppEvent>) {
    mpsc::channel()
}

pub fn wait_for_event(rx: &Receiver<AppEvent>, target_event: AppEvent) {
    while let Ok(event) = rx.recv() {
        if std::mem::discriminant(&event) == std::mem::discriminant(&target_event) {
            break;
        }
    }
}
