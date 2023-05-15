use log::debug;

use temporal_sdk_core::protos::coresdk::activity_result::{
    activity_resolution::Status::Completed, ActivityResolution,
};

pub fn parse_activity_result<'a, T>(result: &'a ActivityResolution) -> Result<T, anyhow::Error>
where
    T: serde::Deserialize<'a>,
{
    if result.completed_ok() {
        if let Some(Completed(result)) = &result.status {
            if let Some(payload) = &result.result {
                // let data = from_utf8(&payload.data).unwrap();
                let result: T = serde_json::from_slice(&payload.data).unwrap();
                // println!("Activity completed with: {:#?}", string_result.to_owned());
                return Ok(result);
            }
        } else {
            debug!("Activity failed with {:?}", result.status);
        }
    }
    Err(anyhow::anyhow!("Activity failed"))
}
