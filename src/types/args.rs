use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Mode {
    Client,
    Server,
}

#[derive(Clone, Debug, Parser)]
#[command(about, version)]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    #[arg(short, long, default_value = "log_config.yml")]
    pub log_config: String,

    #[arg(short, long, default_value = "server")]
    pub mode: Mode,
}
