use std::{str::FromStr, sync::Arc};
use temporal_sdk::{sdk_client_options, Worker};
use temporal_sdk_core::{init_worker, CoreRuntime, Url};
use temporal_sdk_core_api::{telemetry::TelemetryOptionsBuilder, worker::WorkerConfigBuilder};

use crate::activities;
use crate::workflows;

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;

    let client = server_options.connect("default", None, None).await?;

    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("activities-cancellation-heartbeating")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(
        Arc::new(core_worker),
        "activities-cancellation-heartbeating",
    );

    worker.register_activity("fake_progress", activities::fake_progress);
    worker.register_wf(
        "run_cancellable_activity",
        workflows::run_cancellable_activity,
    );

    worker.run().await?;

    Ok(())
}
