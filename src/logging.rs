use log::LevelFilter;
use simplelog::*;
use std::path::PathBuf;

pub fn init_logger() {
    #[cfg(debug_assertions)]
    init_debug_logger();

    #[cfg(not(debug_assertions))]
    init_release_logger();
}

#[cfg(debug_assertions)]
fn init_debug_logger() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap_or_else(|_| {
        SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    });

    log::info!("Logger initialized (debug mode - terminal output)");
}

#[cfg(not(debug_assertions))]
fn init_release_logger() {
    let log_dir = get_log_directory();

    if let Err(e) = fs::create_dir_all(&log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        return;
    }

    let log_file_path = log_dir.join("mozannab.log");

    let log_file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
            return;
        }
    };

    WriteLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new().set_time_format_rfc3339().build(),
        log_file,
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to initialize file logger: {}", e);
    });

    log::info!("Logger initialized (release mode - file output)");
    log::info!("Log file: {}", log_file_path.display());
}

fn get_log_directory() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());

        PathBuf::from(appdata).join("Mozannab").join("logs")
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("Mozannab")
            .join("logs")
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("Mozannab")
            .join("logs")
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from("logs")
    }
}

pub fn get_log_file_path() -> Option<PathBuf> {
    #[cfg(not(debug_assertions))]
    {
        Some(get_log_directory().join("mozannab.log"))
    }

    #[cfg(debug_assertions)]
    {
        None
    }
}

pub fn open_log_directory() {
    #[cfg(not(debug_assertions))]
    {
        let log_dir = get_log_directory();

        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("explorer").arg(log_dir).spawn();
        }

        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").arg(log_dir).spawn();
        }

        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open").arg(log_dir).spawn();
        }
    }
}

pub fn rotate_logs_if_needed() {
    #[cfg(not(debug_assertions))]
    {
        let log_file_path = get_log_directory().join("mozannab.log");

        if let Ok(metadata) = fs::metadata(&log_file_path) {
            const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

            if metadata.len() > MAX_LOG_SIZE {
                let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                let archived_name = format!("mozannab_{}.log", timestamp);
                let archived_path = get_log_directory().join(archived_name);

                if let Err(e) = fs::rename(&log_file_path, &archived_path) {
                    log::error!("Failed to rotate log file: {}", e);
                } else {
                    log::info!("Log file rotated to: {}", archived_path.display());
                }
            }
        }
    }
}
