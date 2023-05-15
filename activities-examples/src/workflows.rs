use log::{debug, info, warn};
use prost_wkt_types::Duration as ProstDuration;
use std::time::Duration;
use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::temporal::api::common::v1::RetryPolicy;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

use crate::helpers::parse_activity_result;

pub async fn http_workflow(ctx: WfContext) -> WorkflowResult<String> {
    debug!("Inside http workflow");
    let act_handle = ctx
        .activity(ActivityOptions {
            activity_type: "make_http_request".to_string(),
            input: "".as_json_payload()?, // no actual payload
            retry_policy: Some(RetryPolicy {
                initial_interval: Some(ProstDuration {
                    seconds: 0,
                    nanos: 50_000_000, // 50ms
                }),
                maximum_attempts: 2,
                ..Default::default()
            }),
            start_to_close_timeout: Some(Duration::from_secs(30)),
            ..Default::default()
        })
        .await;

    match parse_activity_result::<String>(&act_handle) {
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

pub async fn async_activity_workflow(ctx: WfContext) -> WorkflowResult<String> {
    debug!("Inside async activity workflow");
    let act_handle = ctx
        .activity(ActivityOptions {
            activity_type: "do_something_async".to_string(),
            input: "".as_json_payload()?, // no actual payload
            retry_policy: Some(RetryPolicy {
                initial_interval: Some(ProstDuration {
                    seconds: 0,
                    nanos: 50_000_000, // 50ms
                }),
                maximum_attempts: 2,
                ..Default::default()
            }),
            start_to_close_timeout: Some(Duration::from_secs(30)),
            ..Default::default()
        })
        .await;

    match parse_activity_result::<String>(&act_handle) {
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
