use log::{debug, info};

use std::{sync::Arc, time::Duration};
use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core_protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};
use tokio::sync::Mutex;

pub async fn sample_schedule_workflow(ctx: WfContext) -> WorkflowResult<()> {
    println!("Inside sample_schedule_workflow");

    let info = ctx.get_args();
    println!("info {:?}", info);

    // println!("ctx: {:#?}", ctx.get_info());
    for payload in info.iter() {
        println!("payload {:?}", String::from_json_payload(payload).unwrap());
    }
    ctx.timer(Duration::from_secs(5)).await;

    println!("Workflow complete");

    Ok(WfExitValue::Normal(()))
}
