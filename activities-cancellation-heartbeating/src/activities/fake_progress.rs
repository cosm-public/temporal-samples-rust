use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use temporal_sdk::ActContext;
use temporal_sdk_core_protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};

/// Make the http request
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Response {
    args: HashMap<String, String>,
}

pub async fn fake_progress(ctx: ActContext, sleep_interval_ms: u64) -> Result<u64, anyhow::Error> {
    let mut starting_point = match ctx.get_heartbeat_details().get(0) {
        Some(hb) => u64::from_json_payload(hb)?,
        None => 1,
    };

    while starting_point <= 100 {
        info!("Progress: {}", starting_point);
        ctx.record_heartbeat(vec![starting_point.as_json_payload()?]);
        tokio::time::sleep(tokio::time::Duration::from_nanos(
            sleep_interval_ms * 1_000_000,
        ))
        .await;
        starting_point += 1;
    }

    info!("Done with progress!");

    Ok(starting_point)
}
