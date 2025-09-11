use clap::Parser;
use mesh_core::types::args::Args;

#[tokio::main]
async fn main() {
    let arguments = Args::parse();

    match log4rs::init_file(&arguments.log_config, Default::default()) {
        Ok(()) => log::info!("Logger successfully initialized for Plant Go!"),
        Err(e) => log::error!("Logger couldn't be initialized for Plant Go: {}", e),
    }
    log::info!("Mesh Core Initialized!");

    tokio::spawn(async {
        println!("Tick!");
    })
    .await
    .unwrap();
}
