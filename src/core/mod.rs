pub mod calibrator;
pub mod model;
pub mod pose;
pub mod utils;
pub mod videocapture;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    check_interval_ms: u64,
}

pub fn read_check_interval_or_default() -> u64 {
    // try read from config file (only for check interval for now)
    let appdata = std::env::var("APPDATA")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    let config_dir = std::path::PathBuf::from(appdata).join("Mozannab");
    let _ = std::fs::create_dir_all(&config_dir);
    let config_path = config_dir.join("config.json");
    let check_interval_ms = if let Ok(content) = std::fs::read_to_string(&config_path) {
        serde_json::from_str::<ConfigFile>(&content)
            .map(|c| c.check_interval_ms)
            .unwrap_or(60000)
    }
    else {
        // File doesn't exist, create it with default
        let default_config = ConfigFile {
            check_interval_ms: 60000,
        };
        let _ = std::fs::write(
            &config_path,
            serde_json::to_string_pretty(&default_config).unwrap(),
        );
        60000
    };
    
    log::info!("Posture check interval set to {} ms", check_interval_ms);
    return check_interval_ms;
}

struct PostureProcConfig {
    face_proto: &'static str,
    face_model: &'static str,

    // Calibration settings
    calibration_samples_needed: usize,
    mad_filter_k: f64,

    // Posture assessment settings
    check_interval_ms: u64,
    check_interval_jitter_ms: u64,

    // Detection thresholds
    face_conf_threshold: f32,

    // Posture assessment
    bad_posture_duration_secs: f64,
    history_size: usize,
    history_bad_threshold: f64,
    score_bad_threshold: f64,
    score_good_threshold: f64,
}

impl Default for PostureProcConfig {
    fn default() -> Self {
        Self {
            // Model paths
            face_proto: "deploy.prototxt",
            face_model: "res10_300x300_ssd_iter_140000.caffemodel",

            // Calibration settings
            calibration_samples_needed: 10,
            mad_filter_k: 3.5,

            // Posture assessment interval
            check_interval_ms: read_check_interval_or_default(),
            check_interval_jitter_ms: 500,

            // Detection thresholds
            face_conf_threshold: 0.5,

            // Posture assessment
            bad_posture_duration_secs: 3.0,
            history_size: 15,
            history_bad_threshold: 0.6,
            score_bad_threshold: 0.5,
            score_good_threshold: -0.1,
        }
    }
}

#[derive(Debug, Clone)]
struct FaceData {
    area: f64,
    bbox: (i32, i32, i32, i32),
    conf: f32,
}

#[derive(Debug, Clone)]
pub struct CalibrationResult {
    posture_good_baseline: f64,
    posture_bad_baseline: f64,
    posture_midpoint: f64,
}
