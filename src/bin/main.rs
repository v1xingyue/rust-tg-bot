use ruts_tg::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();
    tools::log_init();
    log::info!("Starting telegram bot...");

    bot::MyBot::bootstrap().await;

    Ok(())
}
