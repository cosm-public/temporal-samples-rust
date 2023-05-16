use log::info;
use nanoid::nanoid;
use std::time::Duration;
use temporal_client::WorkflowOptions;
use temporal_helpers::client::get_client;
use temporal_sdk_core::WorkflowClientTrait;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let workflow_id = format!("workflow-id-{}", nanoid!());
    let handle = client
        .start_workflow(
            vec!["".as_json_payload()?.into()],
            "activities-cancellation-heartbeating".to_owned(), // task queue
            workflow_id.to_owned(),                            // workflow id
            "run_cancellable_activity".to_owned(),             // workflow type
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    info!(
        "Started workflow_id: {}, run_id: {}",
        workflow_id, handle.run_id
    );
    info!("Sleeping 30s to allow workflow to run");
    tokio::time::sleep(Duration::from_secs(30)).await;

    info!("Requesting cancellation");
    let _cancel_handle = client
        .cancel_workflow_execution(
            workflow_id,
            Some(handle.run_id),
            "Try and cancel".to_string(),
            None,
        )
        .await?;

    Ok(())
}
