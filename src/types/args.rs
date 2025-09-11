use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(about, version)]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    #[arg(short, long, default_value = "log_config.yml")]
    pub log_config: String,
}
