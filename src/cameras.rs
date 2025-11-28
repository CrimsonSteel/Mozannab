
pub type CameraList = Vec<(i32, String)>;

pub fn enumerate_cameras() -> Result<CameraList, String> {

    let Some(api) = nokhwa::native_api_backend() else {
        return Err("Native camera API is not available".to_string());
    };

    match nokhwa::query(api) {
        Ok(cameras) => {
            let camera_list: CameraList = cameras.into_iter().filter_map(|camera| {

                if camera.index().as_index().is_err() {
                    return None;
                }

                let index = camera.index().as_index().ok()?;

                let name = camera.human_name();
                Some((index as i32, name))
            }).collect();

            return Ok(camera_list);
        }
        Err(e) => {
            return Err(format!("Failed to query cameras: {}", e));
        }
    }
}