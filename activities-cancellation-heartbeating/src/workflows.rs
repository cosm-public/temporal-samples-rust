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

    let fake_progress_handle = ctx.activity(ActivityOptions {
        activity_type: "fake_progress_activity".to_string(),
        cancellation_type: ActivityCancellationType::WaitCancellationCompleted,
        input: 500.as_json_payload().expect("Unable to serialize"), // payload here is the sleep time
        // Note that how long you set the timeout to will impact when the cancellation for the activity is processed
        // if you make it too long, it'll just keep going until the end.
        heartbeat_timeout: Some(Duration::from_secs(3)),
        start_to_close_timeout: Some(Duration::from_secs(120)),
        ..Default::default()
    });

    let skipped_handle = ctx.activity(ActivityOptions {
        activity_type: "skipped_activity".to_string(),
        cancellation_type: ActivityCancellationType::TryCancel,
        input: "".as_json_payload().expect("Unable to serialize"), // empty payload
        heartbeat_timeout: Some(Duration::from_secs(3)),
        start_to_close_timeout: Some(Duration::from_secs(120)),
        ..Default::default()
    });

    let cancel_handle = ctx.cancelled();

    let mut final_value = 0;

    // tokio select waits on multiple asyncs and returns after the FIRST one completes
    tokio::select!(
        _ = cancel_handle => {
            warn!("## workflow canceled ##");
        },
        res = async {
            let progress_result = fake_progress_handle.await;
            // should never get called
            skipped_handle.await;
            progress_result
        } => {
            final_value = parse_activity_result(&res)?;
            info!("Activity handle finished {:?}", res);
        }
    );

    ctx.activity(ActivityOptions {
        activity_type: "cleanup_activity".to_string(),
        cancellation_type: ActivityCancellationType::TryCancel,
        input: "".as_json_payload().expect("Unable to serialize"), // empty payload
        heartbeat_timeout: Some(Duration::from_secs(3)),
        start_to_close_timeout: Some(Duration::from_secs(120)),
        ..Default::default()
    })
    .await;

    info!("In the end the value was: {:?}", final_value);

    Ok(WfExitValue::Normal(final_value))
}
