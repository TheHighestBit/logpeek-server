use logpeek::config::Config;
use logpeek::init;
use log::*;
use logpeek_server::run;

#[tokio::main]
async fn main() {
    // Logger setup
    let config = Config { 
        datetime_format: logpeek::config::DateTimeFormat::ISO8601,
        min_log_level: LevelFilter::Trace,
        ..Default::default() };
    init(config).unwrap();

    info!("Starting...");
    run().await;
}