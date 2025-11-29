use egui::epaint::util;
use opencv::{
    core::{self, Mat, Point, Rect, Scalar, Size, Vector},
    dnn,
    highgui,
    imgproc,
    prelude::*,
    videoio::{self, VideoCapture, VideoCaptureAPIs},
};

use std::collections::VecDeque;
use std::path::Path;
use std::time::Instant;

use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "models"]
struct Assets;

static FACE_NET: OnceCell<Arc<Mutex<dnn::Net>>> = OnceCell::new();

pub fn get_face_net() -> Option<Arc<Mutex<dnn::Net>>> {
    FACE_NET.get().cloned()
}

pub fn get_or_init_face_net(config: &super::PostureProcConfig) -> opencv::Result<Arc<Mutex<dnn::Net>>> {
    if let Some(existing) = FACE_NET.get() {
        return Ok(existing.clone());
    }

    let configs = super::PostureProcConfig::default();
    
    let proto = Assets::get(configs.face_proto).unwrap();
    let model = Assets::get(configs.face_model).unwrap();

    let proto_bytes: Vector<u8> = core::Vector::from_slice(proto.data.as_ref());
    let model_bytes: Vector<u8> = core::Vector::from_slice(model.data.as_ref());

    let mut net = dnn::read_net_from_caffe_buffer(
        &proto_bytes,
        &model_bytes,
    )?;


    net.set_preferable_backend(dnn::DNN_BACKEND_OPENCV)?;  // IMPORTANT
    net.set_preferable_target(dnn::DNN_TARGET_CPU)?;

    println!("Using CPU DNN backend for face_net");


    let arc = Arc::new(Mutex::new(net));
    match FACE_NET.set(arc.clone()) {
        Ok(()) => Ok(arc),
        Err(_) => Ok(FACE_NET.get().unwrap().clone()),
    }
}

pub fn batch_inference(inputs: &Vec<Mat>) -> opencv::Result<Vec<Option<super::FaceData>>> {
    
    let net_guard = get_or_init_face_net(&super::PostureProcConfig::default())?;
    let net_locked = net_guard.lock();

    let Ok(mut net) = net_locked else {
        return Err(opencv::Error::new(opencv::core::StsError, "Failed to lock face net mutex"));
    };

    // resize and prepare blob
    let resized_inputs: core::Vector<Mat> = inputs.iter().map(|img| {
        let resized = super::utils::resize_image(img, core::Size::new(300, 300)).unwrap();
        resized
    }).collect();

    let blob = dnn::blob_from_images(
        &resized_inputs,
        1.0,
        core::Size::new(300, 300),
        core::Scalar::new(104.0, 177.0, 123.0, 0.0),
        false,
        false,
        opencv::core::CV_32F,
    ).unwrap();

    /*
    println!("Blob dims: {}", blob.dims());
    println!("Blob size (2D view): {:?}", blob.size()?);
    
    let shape = blob.mat_size();
    println!("Blob FULL shape: [");
    for i in 0..shape.len() {
        println!("  dim[{}] = {}", i, shape[i]);
    }
    println!("]");
    println!("Expected: [batch={}, channels=3, height=300, width=300]", resized_inputs.len());
    */

    net.set_input(&blob, "", 1.0, core::Scalar::default()).unwrap();
    let detections = net.forward_single("").unwrap();

    // net.forward_layer(&mut detections, &output_layers.get(0).unwrap()).unwrap();

    let mat_size = detections.mat_size();
    let num_detections = mat_size[2];
    let detections_2d = detections.reshape(1, num_detections)?;

    let batch_size = inputs.len();
    let mut batch_results: Vec<Option<super::FaceData>> = vec![None; batch_size];

    let (w, h) = inputs.get(0).map(|img| (img.cols(), img.rows())).unwrap_or((0, 0));

    for i in 0..num_detections {
        // Extract the 7 values from the 2D Mat (now it's [num_detections, 7])
        let image_id = *detections_2d.at_2d::<f32>(i, 0)? as usize;
        let class_id = *detections_2d.at_2d::<f32>(i, 1)?;
        let confidence = *detections_2d.at_2d::<f32>(i, 2)?;
        let x1_norm = *detections_2d.at_2d::<f32>(i, 3)?;
        let y1_norm = *detections_2d.at_2d::<f32>(i, 4)?;
        let x2_norm = *detections_2d.at_2d::<f32>(i, 5)?;
        let y2_norm = *detections_2d.at_2d::<f32>(i, 6)?;
        
        // Skip if confidence is too low
        if confidence < 0.5 {
            continue;
        }
        
        if image_id >= batch_size {
            continue;
        }
        
        // Convert normalized coordinates to pixel coordinates
        let x1 = (x1_norm * w as f32) as i32;
        let y1 = (y1_norm * h as f32) as i32;
        let x2 = (x2_norm * w as f32) as i32;
        let y2 = (y2_norm * h as f32) as i32;
        
        let bw = (x2 - x1).max(0);
        let bh = (y2 - y1).max(0);
        
        let normalized_area = (bw as f32 / w as f32) * (bh as f32 / h as f32);
        
        // Keep only the best detection for each image
        let should_update = match &batch_results[image_id] {
            None => true,
            Some(existing) => confidence > existing.conf,
        };
        
        if should_update {
            batch_results[image_id] = Some(super::FaceData {
                area: normalized_area as f64,
                bbox: (x1, y1, bw, bh),
                conf: confidence,
            });
        }
    }

    Ok(batch_results)
}
