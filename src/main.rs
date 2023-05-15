mod args;
mod prayer_utils;
mod settings;

use args::{CliArgs, CliSubcommand, NextCommand, TimeCommand};
use clap::Parser;
use settings::init;

fn handle_next_prayer(params: NextCommand) {
    todo!("print next prayer time left ({})", params.time_left)
}

fn handle_prayer_time(params: TimeCommand) {
    todo!("print {}", params.prayer_name)
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.sub_command {
        CliSubcommand::Init(_) => init().await,
        CliSubcommand::Next(params) => handle_next_prayer(params),
        CliSubcommand::Time(params) => handle_prayer_time(params),
    }
}
