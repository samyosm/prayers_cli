mod args;
mod prayer_utils;
mod settings;

use args::{CliArgs, CliSubcommand, NextCommand, TimeCommand};
use clap::Parser;
use prayer_utils::{get_prayer_times, string_to_prayer};
use salah::Utc;
use settings::init;

use crate::settings::get_formated_date;

fn handle_next_prayer(params: NextCommand) {
    todo!("print next prayer time left ({})", params.time_left)
}

fn handle_prayer_time(params: TimeCommand) {
    let prayer = string_to_prayer(&params.prayer_name);
    let prayers = get_prayer_times();
    let prayer_time = prayers.time(prayer);
    let remaining_time = prayer_time.signed_duration_since(Utc::now());
    match params.time_left {
        true => println!(
            "in {}h {}m",
            remaining_time.num_hours(),
            remaining_time.num_minutes() % 60
        ),
        false => println!("{}", get_formated_date(prayer_time)),
    }
}

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    let args = CliArgs::parse();

    match args.sub_command {
        CliSubcommand::Init(_) => init().await,
        CliSubcommand::Next(params) => handle_next_prayer(params),
        CliSubcommand::Time(params) => handle_prayer_time(params),
    };

    Ok(())
}
