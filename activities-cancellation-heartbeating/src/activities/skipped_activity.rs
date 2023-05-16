use log::info;
use temporal_sdk::ActContext;

pub async fn skipped_activity(
    _ctx: ActContext,
    _payload: Option<String>,
) -> Result<(), anyhow::Error> {
    info!("Starting skipped activity");

    Ok(())
}
