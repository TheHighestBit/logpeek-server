use log::trace;

pub async fn hello_world() -> String {
    trace!("Hello world request received");
    "Hello, world!".to_string()
}