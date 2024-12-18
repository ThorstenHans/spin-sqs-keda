use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long, default_value = 1_000)]
    pub message_count: u16,

    #[arg(short, long)]
    pub queue_url: String,
}
