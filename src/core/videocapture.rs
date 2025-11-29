use opencv::{
    core::{self, Mat, Point, Rect, Scalar, Size, Vector},
    dnn::{self, Net},
    highgui,
    imgproc,
    prelude::*,
    videoio::{self, VideoCapture, VideoCaptureAPIs},
};

pub fn open_camera(index: i32, api: Option<VideoCaptureAPIs>) -> opencv::Result<VideoCapture> {
    let mut cam = VideoCapture::new(index, api.unwrap_or(VideoCaptureAPIs::CAP_ANY).into() )?;
    if !cam.is_opened()? {
        return Err(opencv::Error::new(opencv::core::StsError, format!("Failed to open camera with index {}", index)));
    }

    // First few frames can sometimes be black, so we need to discard them
    const DISCARDED_FRAMES: i32 = 5;
    for _ in 0..DISCARDED_FRAMES {
        let mut temp_frame = Mat::default();
        cam.read(&mut temp_frame)?;
    }

    Ok(cam)
}

pub fn close_camera(cam: &mut VideoCapture) -> opencv::Result<()> {
    cam.release()?;
    Ok(())
}

pub fn read_frame(cam: &mut VideoCapture, flip: bool) -> opencv::Result<Mat> {
    let mut frame = Mat::default();
    cam.read(&mut frame)?;
    if frame.empty() {
        return Err(opencv::Error::new(opencv::core::StsError, "Failed to read frame from camera"));
    }
    if flip {
        core::flip(&frame.clone(), &mut frame, 1)?;
    }
    Ok(frame)
}

