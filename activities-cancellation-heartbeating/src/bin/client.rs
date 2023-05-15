use nanoid::nanoid;
use temporal_client::WorkflowOptions;
use temporal_helpers::client::get_client;
use temporal_sdk_core::WorkflowClientTrait;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client().await?;

    let _handle = client
        .start_workflow(
            vec!["".as_json_payload()?.into()],
            "activities-cancellation-heartbeating".to_owned(), // task queue
            format!("workflow-id-{}", nanoid!()),              // workflow id
            "run_cancellable_activity".to_owned(),             // workflow type
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}
