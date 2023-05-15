use activities_examples::client::get_client;
use nanoid::nanoid;

use temporal_client::WorkflowOptions;
use temporal_sdk_core::WorkflowClientTrait;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client().await?;

    let _handle1 = client
        .start_workflow(
            vec!["".as_json_payload()?.into()],
            "activities-examples".to_owned(),     // task queue
            format!("workflow-id-{}", nanoid!()), // workflow id
            "http_workflow".to_owned(),           // workflow type
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    let _handle2 = client
        .start_workflow(
            vec!["".as_json_payload()?.into()],
            "activities-examples".to_owned(),     // task queue
            format!("workflow-id-{}", nanoid!()), // workflow id
            "async_activity_workflow".to_owned(), // workflow type
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}
