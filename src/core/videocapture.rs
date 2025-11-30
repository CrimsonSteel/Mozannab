use opencv::{
    core::{self, Mat},
    prelude::*,
    videoio::{VideoCapture, VideoCaptureAPIs},
};

pub fn open_camera(index: i32, api: Option<VideoCaptureAPIs>) -> opencv::Result<VideoCapture,opencv::Error> {

    let used_api: i32;
    // if windows, use DirectShow by default, elese, use any
    #[cfg(target_os = "windows")]
    {
        used_api = api.unwrap_or(VideoCaptureAPIs::CAP_DSHOW).into();
    }
    #[cfg(not(target_os = "windows"))]
    {
        used_api = api.unwrap_or(VideoCaptureAPIs::CAP_ANY).into();
    }

    let mut cam = VideoCapture::new(index, used_api)?;
    if !cam.is_opened()? {
        
        return Err(opencv::Error::new(
            opencv::core::StsError,
            format!("Failed to open camera with index {index}"),
        ));
    }

    // Wait for non uniform frames
    const TIMEOUT_FRAMES : i32 = 32;

    for index in 0..TIMEOUT_FRAMES {
        let mut temp_frame = Mat::default();

        let read = cam.read(&mut temp_frame)?;
        if !read || temp_frame.empty() {
            log::warn!("Camera {index} read empty frame during initialization check");
            continue;
        }
        let (is_uniform, std0, std1, std2) = super::utils::is_frame_uniform_color(&temp_frame, 1.0)?;
        println!("Camera {index} initialization check: is_uniform={is_uniform}, stddev=({std0}, {std1}, {std2})");
        if !is_uniform {
            log::debug!("Camera {index} initialized after {} frames", index + 1);
            break;
        }
    }
    if index == TIMEOUT_FRAMES - 1 {
        log::warn!("Camera {index} may not have initialized properly after {} frames", TIMEOUT_FRAMES);
        return Err(opencv::Error::new(
            opencv::core::StsError,
            format!("Camera {index} may not have initialized properly after {} frames", TIMEOUT_FRAMES),
        ));
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
        return Err(opencv::Error::new(
            opencv::core::StsError,
            "Failed to read frame from camera",
        ));
    }
    if flip {
        core::flip(&frame.clone(), &mut frame, 1)?;
    }
    Ok(frame)
}
