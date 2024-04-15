use logpeek_server::run;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("\x1b[0m");
        println!("Terminal colors reset. Press Ctrl+C again to exit.");
    });
    
    run().await;
}