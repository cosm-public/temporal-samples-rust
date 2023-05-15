use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, time::Duration};
use temporal_client::WorkflowClientTrait;
use temporal_sdk::{ActContext, ActExitValue};
use temporal_sdk_core::TaskToken;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;
use tokio::sync::Mutex;
use tokio::time::sleep;

use temporal_helpers::client::get_client;

/// Make the http request
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Response {
    args: HashMap<String, String>,
}

pub async fn do_something_async(
    ctx: ActContext,
    _payload: Option<String>,
) -> Result<ActExitValue<()>, anyhow::Error> {
    let shared_token: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let shared_token_ref = shared_token.clone();
    {
        // set the `activity_task_token`
        let activity_info = ctx.get_info();
        let task_token = &activity_info.task_token;
        let mut shared = shared_token_ref.lock().await;
        *shared = Some(task_token.clone());

        debug!("Task token: {:?}", task_token);
    }

    // tokio::task::spawn(do_some_work(task_token));
    let shared_token_ref2 = shared_token.clone();
    tokio::spawn(do_some_work(shared_token_ref2));
    info!("Do some work spawned");
    Ok::<ActExitValue<()>, _>(ActExitValue::WillCompleteAsync)
}

async fn do_some_work(token_ref: Arc<Mutex<Option<Vec<u8>>>>) -> Result<(), anyhow::Error> {
    info!("Starting doing some work...and sleeping");
    sleep(Duration::from_secs(5)).await;

    let client = get_client().await?;

    info!("Trying to mark as complete...");
    loop {
        let mut shared = token_ref.lock().await;
        let maybe_token = shared.take();

        if let Some(task_token) = maybe_token {
            client
                .complete_activity_task(
                    TaskToken(task_token),
                    Some("test".as_json_payload().unwrap().into()),
                )
                .await
                .unwrap();
            return Ok(());
        }
    }
}
