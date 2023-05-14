mod args;
mod init;

use args::CliArgs;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    println!("{:?}", args);
}
