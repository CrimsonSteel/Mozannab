use opencv::{
    core::{self, Mat, Point, Rect, Scalar, Size, Vector},
    dnn::{self, Net},
    highgui,
    imgproc,
    prelude::*,
    videoio::{self, VideoCapture, VideoCaptureAPIs},
};

pub fn resize_image(img: &Mat, new_size: Size) -> opencv::Result<Mat> {
    let mut resized = Mat::default();
    imgproc::resize(
        img,
        &mut resized,
        new_size,
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )?;
    Ok(resized)
}

pub fn show_image(window_name: &str, img: &Mat, delay: i32) -> opencv::Result<()> {
    highgui::imshow(window_name, img)?;
    highgui::wait_key(delay)?;
    Ok(())
}

pub fn draw_detections(images: &mut Vec<Mat>, detections: &Vec<super::FaceData>) -> opencv::Result<()> {
    if images.len() != detections.len() {
        return Err(opencv::Error::new(
            opencv::core::StsError,
            "Number of images must match number of detections",
        ));
    }

    for (img, detection) in images.iter_mut().zip(detections.iter()) {
        let (x, y, w, h) = detection.bbox;
        
        let rect = Rect::new(x, y, w, h);
        
        let green = Scalar::new(0.0, 255.0, 0.0, 0.0);
        imgproc::rectangle(img, rect, green, 2, imgproc::LINE_8, 0)?;
        
        let conf_text = format!("{:.2}", detection.conf);
        let font = imgproc::FONT_HERSHEY_SIMPLEX;
        let font_scale = 0.6;
        let thickness = 2;
        
        let text_pos = Point::new(x, y - 10);
        
        let text_size = core::Size::default();
        let mut baseline = 0;
        imgproc::get_text_size( &conf_text, font, font_scale, thickness, &mut baseline)?;
        
        let text_bg_rect = Rect::new(
            x,
            y - text_size.height - 10,
            text_size.width,
            text_size.height + 10,
        );
        
        let black = Scalar::new(0.0, 0.0, 0.0, 0.0);
        imgproc::rectangle(img, text_bg_rect, black, -1, imgproc::LINE_8, 0)?;

        let white = Scalar::new(255.0, 255.0, 255.0, 0.0);
        imgproc::put_text(img, &conf_text, text_pos, font, font_scale, white, thickness, imgproc::LINE_8, false)?;
    }

    Ok(())
}

pub fn mad_filter_median(values: &[f64], k: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    };
    
    // Calculate MAD
    let mut deviations: Vec<f64> = values.iter().map(|v| (v - median).abs()).collect();
    deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mad = if deviations.len() % 2 == 0 {
        (deviations[deviations.len() / 2 - 1] + deviations[deviations.len() / 2]) / 2.0
    } else {
        deviations[deviations.len() / 2]
    } + 1e-9;

    // filter outliers
    let filtered: Vec<f64> = values
        .iter()
        .filter(|v| ((**v - median).abs() / mad) < k)
        .copied()
        .collect();

    if filtered.is_empty() {
        return median;
    }

    let mut sorted_filtered = filtered;
    sorted_filtered.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if sorted_filtered.len() % 2 == 0 {
        (sorted_filtered[sorted_filtered.len() / 2 - 1] + sorted_filtered[sorted_filtered.len() / 2]) / 2.0
    } else {
        sorted_filtered[sorted_filtered.len() / 2]
    }

}   
