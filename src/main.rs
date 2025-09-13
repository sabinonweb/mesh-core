use clap::Parser;
use mesh_core::{
    types::args::{Args, Mode},
    udpsocket::{client, server},
};
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
async fn task(_name: &str, interval: u64) {
    loop {
        println!("Task {} tick", interval);
        sleep(Duration::from_secs(interval)).await;
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let arguments = Args::parse();

    match log4rs::init_file(&arguments.log_config, Default::default()) {
        Ok(()) => log::info!("Logger successfully initialized for Plant Go!"),
        Err(e) => log::error!("Logger couldn't be initialized for Plant Go: {}", e),
    }
    log::info!("Mesh Core Initialized!");

    match arguments.mode {
        Mode::Server => {
            log::info!("Starting server...");
            server().await?;
        }
        Mode::Client => {
            log::info!("Starting client...");
            client().await?;
        }
    }
    Ok(())

    // let task1 = tokio::spawn(task("A", 1));
    // let task2 = tokio::spawn(task("B", 2));
    // let task3 = tokio::spawn(task("C", 3));
    //
    // // join lets us await on multiple futures
    // let _ = tokio::join!(task1, task2, task3);
}
