use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(
    author = "bujosa",
    about = "This is a simple app with clap, for learning purposes",
    display_name = "chara",
    disable_help_subcommand = false,
    version
)]
struct Opts {
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    build: bool,
}

pub fn run() {
    let cli = Opts::parse();

    let res = match cli {
        Opts { debug: true, .. } => "Debug mode",
        Opts { build: true, .. } => "Build mode",
        _ => "No mode",
    };

    println!("{}", res);
}
