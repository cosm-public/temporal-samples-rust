use std::sync::Arc;
use temporal_helpers::client::get_client;
use temporal_sdk::Worker;
use temporal_sdk_core::{init_worker, CoreRuntime};
use temporal_sdk_core_api::{telemetry::TelemetryOptionsBuilder, worker::WorkerConfigBuilder};

use crate::workflows;

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client().await?;

    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("schedule")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "schedule");

    worker.register_wf("schedule", workflows::sample_schedule_workflow);

    worker.run().await?;

    Ok(())
}
