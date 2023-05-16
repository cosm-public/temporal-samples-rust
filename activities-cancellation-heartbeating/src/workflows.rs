use log::{debug, info, warn};

use std::time::Duration;
use temporal_helpers::parse_activity_result;
use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core_protos::coresdk::{
    workflow_commands::ActivityCancellationType, AsJsonPayloadExt,
};

pub async fn run_cancellable_activity(mut ctx: WfContext) -> WorkflowResult<u64> {
    info!("Inside run_cancellable_activity");
    debug!("Creating activity handle");

    let act_handle = ctx.activity(ActivityOptions {
        activity_type: "fake_progress".to_string(),
        cancellation_type: ActivityCancellationType::TryCancel,
        input: 500.as_json_payload().expect("Unable to serialize"), // payload here is the sleep time
        heartbeat_timeout: Some(Duration::from_secs(3)),
        start_to_close_timeout: Some(Duration::from_secs(120)),
        ..Default::default()
    });

    let cancel_handle = ctx.cancelled();

    let mut final_value = 0;
    tokio::select!(
        _ = cancel_handle => {
            warn!("## workflow canceled ##");
        },
        res = act_handle => {
            final_value = parse_activity_result(&res)?;
            info!("Activity handle finished {:?}", res);
        }
    );

    info!("after select the value was: {:?}", final_value);

    Ok(WfExitValue::Normal(final_value))
}
