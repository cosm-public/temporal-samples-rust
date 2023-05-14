mod activities;

use activities::helpers::create_payload;

use nanoid::nanoid;

use std::str::FromStr;
use temporal_client::WorkflowOptions;
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::{Url, WorkflowClientTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;

    let client = server_options.connect("default", None, None).await?;

    let _handle = client
        .start_workflow(
            vec![create_payload(())],
            "activities-examples".to_owned(),
            format!("workflow-id-{}", nanoid!()),
            "http_workflow".to_owned(),
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}
