use std::time::Duration;

use log::debug;
use nanoid::nanoid;

use prost_wkt_types::{Duration as ProstDuration, Timestamp};
use temporal_client::ScheduleOptions;
use temporal_helpers::client::get_client;
use temporal_sdk_core::WorkflowClientTrait;
use temporal_sdk_core_protos::temporal::api::{
    enums::v1::ScheduleOverlapPolicy,
    schedule::v1::{IntervalSpec, SchedulePolicies, ScheduleSpec, TriggerImmediatelyRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let schedule_id = format!("schedule-id-{}", nanoid!());
    let workflow_id = format!("workflow-id-{}", nanoid!());

    // create the schedule
    client
        .create_schedule(
            schedule_id.clone(),
            None,
            ScheduleOptions {
                workflow_id: Some(workflow_id.clone()),
                workflow_type: Some("schedule".to_string()),
                task_queue: Some("schedule".to_string()),
                spec: Some(ScheduleSpec {
                    cron_string: vec!["@every 10s".to_string()],
                    start_time: Some(Timestamp::date_time(2024, 5, 18, 20, 0, 0)?),
                    end_time: Some(Timestamp::date_time(2024, 5, 18, 22, 0, 0)?),
                    ..Default::default()
                }),
                paused: Some(true),
                remaining_actions: Some(3),
                policies: Some(SchedulePolicies {
                    overlap_policy: ScheduleOverlapPolicy::BufferAll.into(),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?;

    println!("Schedule created: {}", schedule_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    // get the current schedule information
    let schedule_response = client.describe_schedule(schedule_id.clone()).await?;

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

        client
            .update_schedule(
                schedule_id.clone(),
                schedule.to_owned(),
                schedule_response.conflict_token,
                None,
            )
            .await?;

        debug!("{:?} post update", schedule);
    }

    println!("Schedule updated: {}", schedule_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    // trigger the schedule manually...
    client
        .patch_schedule(
            schedule_id.clone(),
            Some(TriggerImmediatelyRequest {
                overlap_policy: ScheduleOverlapPolicy::AllowAll.into(),
            }),
            None,
            None,
            None,
            None,
        )
        .await?;

    println!("Schedule manually triggered: {}", schedule_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    // unpause the schedule
    client
        .patch_schedule(
            schedule_id.clone(),
            None,
            None,
            Some("Unpausing for now!".to_string()),
            None,
            None,
        )
        .await?;

    println!("Schedule unpaused: {}", schedule_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    client.delete_schedule(schedule_id.clone()).await?;

    println!("Schedule deleted: {}", schedule_id);

    Ok(())
}
