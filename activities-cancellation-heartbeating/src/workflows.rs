use log::{debug, info, warn};
use std::time::Duration;
use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

use temporal_helpers::parse_activity_result;

// enum
pub async fn run_cancellable_activity(ctx: WfContext) -> WorkflowResult<u64> {
    debug!("Inside run cancellable activity workflow");
    let act_handle = ctx
        .activity(ActivityOptions {
            activity_type: "fake_progress".to_string(),
            // cancellation_type:
            input: 500.as_json_payload()?, // payload here is the sleep time
            heartbeat_timeout: Some(Duration::from_secs(3)),
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        })
        .await;

    match parse_activity_result::<u64>(&act_handle) {
        Ok(result) => {
            info!("Activity completed with: {:#?}", result);
            Ok(WfExitValue::Normal(result))
        }
        Err(_) => {
            warn!("Activity failed");
            Ok(WfExitValue::Evicted)
        }
    }
}
