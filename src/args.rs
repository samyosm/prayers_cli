use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub sub_command: CliSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    /// Attempts to initialize this utility with your coordinates.
    Init(InitCommand),

    /// Displays the next prayer to begin
    Next(NextCommand),

    /// Gives the beginning time or the time left for a prayer.
    Time(TimeCommand),
}

#[derive(Debug, Args)]
pub struct InitCommand {}

#[derive(Debug, Args)]
pub struct NextCommand {
    /// Displays the time left before that prayer
    #[clap(short = 'l', long = "left")]
    pub time_left: bool,
}

#[derive(Debug, Args)]
pub struct TimeCommand {
    /// The name of the prayer. I.e. Asr, Dhuhr, Asr, Magrib, or Isha.
    pub prayer_name: String,

    /// Displays the time left before that prayer
    #[clap(short = 'l', long = "left")]
    pub time_left: bool,
}
