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
    info!("Starting fake progress activity");

    let starting_point = match ctx.get_heartbeat_details().get(0) {
        Some(hb) => u64::from_json_payload(hb)?,
        None => 1,
    };

    let cancel_handle = ctx.cancelled();
    let ping_handle = ping(ctx.to_owned(), starting_point, sleep_interval_ms);

    // wait for either the ping or the cancel handle to finish
    let mut value = 0;
    tokio::select!(
        res = ping_handle => {
            println!("### Activity finished ###");
            value = res;
        },
        _ = cancel_handle => {
            // get the last value from the heartbeat
            println!("### Activity canceled <cancel handle> ###");
        }
    );

    Ok(value)
}

async fn ping(ctx: ActContext, starting_point: u64, sleep_interval_ms: u64) -> u64 {
    let mut count = starting_point;
    while count <= 100 {
        println!("Progress: {}", count);

        if ctx.is_cancelled() {
            println!("### Activity canceled <inside ping> ###");
            break;
        }

        ctx.record_heartbeat(vec![count
            .as_json_payload()
            .expect("Couldn't serialize heartbeat")]);

        tokio::time::sleep(std::time::Duration::from_millis(sleep_interval_ms)).await;

        count += 1;
    }
    return count;
}
