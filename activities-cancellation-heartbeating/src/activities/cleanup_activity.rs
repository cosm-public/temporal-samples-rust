use log::info;
use temporal_sdk::ActContext;

pub async fn cleanup_activity(
    _ctx: ActContext,
    _payload: Option<String>,
) -> Result<(), anyhow::Error> {
    info!("Starting cleanup activity");

    Ok(())
}
