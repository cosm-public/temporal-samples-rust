use log::{debug, info};

use std::{sync::Arc, time::Duration};
use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;
use tokio::sync::Mutex;

pub async fn sample_timer_workflow(
    ctx: WfContext,
    processing_threshold_seconds: u64,
) -> WorkflowResult<()> {
    debug!("Inside sample_timer_workflow");

    // In this sample case, we want to demo a use case where the workflow starts
    // a long running order processing operation and in the case that the processing
    // takes too long, we want to send out a notification email to user about the delay,
    // but we won't cancel the operation. If the operation finishes before the timer fires,
    // then we want to cancel the timer.

    let order_processing_handle = ctx.activity(ActivityOptions {
        activity_type: "order_processing_activity".to_string(),
        input: "".as_json_payload().expect("Unable to serialize"), // empty payload
        start_to_close_timeout: Some(Duration::from_secs(10)),
        ..Default::default()
    });

    debug!(
        "Processing threshold: {} seconds",
        processing_threshold_seconds
    );

    // Note:
    // You need to use the ctx timer and not a tokio one. If you use a tokio one,
    // your entire workflow will block waiting for a response from the await.
    let timer_handle = ctx.timer(Duration::from_secs(processing_threshold_seconds));

    let shared_lock: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let timer_lock = shared_lock.clone();
    let processing_lock = shared_lock.clone();

    // join waits for both to finish
    tokio::join!(
        async {
            order_processing_handle.await;
            let mut temp = processing_lock.lock().await;
            *temp = true;
        },
        async {
            timer_handle.await;
            let processing_done = {
                let temp = timer_lock.lock().await;
                *temp
            };
            debug!(
                "## timer completed - processing_done: {} ##",
                processing_done
            );
            // timer completed, so we need to send out a notification email
            if !processing_done {
                ctx.activity(ActivityOptions {
                    activity_type: "send_email_activity".to_string(),
                    input: "".as_json_payload().expect("Unable to serialize"), // empty payload
                    start_to_close_timeout: Some(Duration::from_secs(10)),
                    ..Default::default()
                })
                .await;
            }
        }
    );

    info!("Workflow complete");

    Ok(WfExitValue::Normal(()))
}
