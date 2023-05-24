use std::time::Duration;

use log::debug;
use nanoid::nanoid;
use prost_wkt_types::{Duration as ProstDuration, Timestamp};
use temporal_helpers::client::get_client;
use temporal_sdk_core_protos::temporal::api::{
    common::v1::WorkflowType,
    enums::v1::{ScheduleOverlapPolicy, TaskQueueKind},
    schedule::v1::{
        IntervalSpec, Schedule, ScheduleAction, SchedulePatch, SchedulePolicies, ScheduleSpec,
        ScheduleState, TriggerImmediatelyRequest,
    },
    taskqueue::v1::TaskQueue,
    workflow::v1::NewWorkflowExecutionInfo,
    workflowservice::v1::{
        CreateScheduleRequest, DeleteScheduleRequest, DescribeScheduleRequest,
        PatchScheduleRequest, UpdateScheduleRequest,
    },
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let namespace = "default";
    let schedule_id = format!("schedule-id-{}", nanoid!());
    let workflow_id = format!("workflow-id-{}", nanoid!());

    let mut raw_client = client.get_client().raw_client().to_owned();

    // create the schedule with the raw client
    raw_client.create_schedule(
        CreateScheduleRequest {
            namespace: namespace.to_owned(),
            schedule: Some(Schedule {
                spec: Some(ScheduleSpec {
                    cron_string: vec!["@every 10s".to_string()],
                    start_time: Some(Timestamp::date_time(2024, 5, 18, 20, 0, 0)?),
                    end_time: Some(Timestamp::date_time(2024, 5, 18, 22, 0, 0)?),
                    ..Default::default()
                }),
                action: Some(ScheduleAction {
                    action: Some(temporal_sdk_core_protos::temporal::api::schedule::v1::schedule_action::Action::StartWorkflow(NewWorkflowExecutionInfo {
                        workflow_id: workflow_id.clone(),
                        workflow_type: Some(WorkflowType {
                            name: "schedule".to_string(),
                        }),
                        task_queue: Some(TaskQueue {
                            name: "schedule".to_string(),
                            kind: TaskQueueKind::Unspecified as i32,
                        }),
                        ..Default::default()
                    })),
                }),
                policies: Some(SchedulePolicies {
                    overlap_policy: ScheduleOverlapPolicy::BufferAll.into(),
                    ..Default::default()
                }),
                state: Some(ScheduleState {
                    paused: true,
                    ..Default::default()
                }),
            }),
            schedule_id: schedule_id.clone(),
            request_id: Uuid::new_v4().to_string(),
            ..Default::default()
        }
    ).await?;

    println!("Schedule created: {}", schedule_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    // get the current schedule information
    let schedule_response = raw_client
        .describe_schedule(DescribeScheduleRequest {
            namespace: namespace.to_owned(),
            schedule_id: schedule_id.clone(),
        })
        .await?
        .into_inner();

    // update the schedule by changing the interval
    if let Some(mut schedule) = schedule_response.schedule {
        debug!("{:?} preupdate", schedule);
        let mut spec: ScheduleSpec = schedule.spec.unwrap().to_owned();
        spec.interval = vec![IntervalSpec {
            interval: Some(ProstDuration {
                seconds: 30,
                ..Default::default()
            }),
            ..Default::default()
        }];
        schedule.spec = Some(spec);

        let schedule = raw_client
            .update_schedule(UpdateScheduleRequest {
                namespace: namespace.to_owned(),
                schedule_id: schedule_id.clone(),
                schedule: Some(schedule),
                conflict_token: schedule_response.conflict_token,
                request_id: Uuid::new_v4().to_string(),
                ..Default::default()
            })
            .await?;

        debug!("{:?} post update", schedule);
    }

    println!("Schedule updated: {}", schedule_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    // trigger the schedule manually...
    raw_client
        .patch_schedule(PatchScheduleRequest {
            namespace: namespace.to_owned(),
            schedule_id: schedule_id.clone(),
            patch: Some(SchedulePatch {
                trigger_immediately: Some(TriggerImmediatelyRequest {
                    overlap_policy: ScheduleOverlapPolicy::AllowAll.into(),
                }),
                ..Default::default()
            }),
            request_id: Uuid::new_v4().to_string(),
            ..Default::default()
        })
        .await?;

    println!("Schedule manually triggered: {}", schedule_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    // unpause the schedule

    raw_client
        .patch_schedule(PatchScheduleRequest {
            namespace: namespace.to_owned(),
            schedule_id: schedule_id.clone(),
            patch: Some(SchedulePatch {
                trigger_immediately: None,
                unpause: "Unpausing for now!".to_string(),
                ..Default::default()
            }),
            request_id: Uuid::new_v4().to_string(),
            ..Default::default()
        })
        .await?;

    println!("Schedule unpaused: {}", schedule_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    raw_client
        .delete_schedule(DeleteScheduleRequest {
            namespace: namespace.to_owned(),
            schedule_id: schedule_id.clone(),
            ..Default::default()
        })
        .await?;

    println!("Schedule deleted: {}", schedule_id);

    Ok(())
}
