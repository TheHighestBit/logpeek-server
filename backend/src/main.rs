use logpeek::config::{Config, OutputDirName};
use logpeek::init;
use log::*;
use logpeek::config::LoggingMode::FileAndConsole;
use logpeek_server::run;

#[tokio::main]
async fn main() {
    // Logger setup
    let config = Config { 
        datetime_format: logpeek::config::DateTimeFormat::ISO8601,
        min_log_level: LevelFilter::Trace,
        out_dir_name: OutputDirName::CustomDir("logpeek-logs"),
        logging_mode: FileAndConsole,
        ..Default::default() };
    init(config).unwrap();

    info!("Starting...");
    run().await;
}