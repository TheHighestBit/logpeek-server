use logpeek::config::Config;
use logpeek::init;
use log::*;

fn main() {
    let config = Config { 
        datetime_format: logpeek::config::DateTimeFormat::ISO8601,
        ..Default::default() };
    init(config).unwrap();

    error!("This is an error message");
}