use dotenv::dotenv;

use activities_cancellation_heartbeating::worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    worker::start_worker().await?;
    Ok(())
}
