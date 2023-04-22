use clap::{command, Parser, Subcommand};

use crate::cli::build::{self, BuildCommand};

#[derive(Parser, Debug)]
#[command(
    author = "bujosa",
    about = "This is a simple app with clap, for learning purposes",
    display_name = "chara",
    disable_help_subcommand = true,
    version
)]
struct Opts {
    #[arg(short, long, help = "Activate debug mode")]
    debug: bool,

    #[arg(short, long, help = "Realize build mode")]
    build: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[command()]
enum Command {
    /// With build u can realize a build
    #[command()]
    Build(BuildCommand),
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum NoSubCommand {}

pub fn run() {
    let cli = Opts::parse();

    match cli.command {
        Command::Build(cmd) => {
            build::parse(cmd);
        }
    };
}
