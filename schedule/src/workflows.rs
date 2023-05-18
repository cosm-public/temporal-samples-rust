use log::{debug, info};

use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};

pub async fn sample_schedule_workflow(ctx: WfContext) -> WorkflowResult<()> {
    info!("Inside sample_schedule_workflow");

    let info = ctx.get_args();
    debug!("info {:?}", info);

    info!("Workflow complete");

    Ok(WfExitValue::Normal(()))
}
