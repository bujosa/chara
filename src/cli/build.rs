use clap::{command, Args, Parser, Subcommand};

use crate::cmd;

#[derive(Parser, Debug)]
#[command()]
pub struct BuildCommand {
    /// Indicate if the build should be active
    #[arg(long, short)]
    active_build: Option<bool>,

    #[command(subcommand)]
    command: BuildSubCommand,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum BuildSubCommand {
    /// Indicate if the build should be deleted
    Delete(GetDeleteArgs),
}

#[derive(Args, Debug)]
pub struct GetDeleteArgs {
    /// Indicate if the build should be truly deleted
    #[arg(
        long,
        short,
        help = "Truly delete the build",
        required_if_eq("local", "true")
    )]
    truly: Option<bool>,

    /// Indicate if the build should be deleted
    /// from the local machine
    /// or from the remote machine
    #[arg(long, short)]
    local: Option<bool>,
}

pub fn parse(sub_command: BuildCommand) {
    let BuildCommand {
        active_build,
        command,
    } = sub_command;

    if let Some(active_build) = active_build {
        if active_build {
            match command {
                BuildSubCommand::Delete(args) => {
                    let GetDeleteArgs { truly, local } = args;
                    if let Some(truly) = truly {
                        cmd::build::build_truly(truly);
                    } else if let Some(local) = local {
                        cmd::build::build_local(local)
                    } else {
                        cmd::build::build();
                    }
                }
            }
        } else {
            cmd::build::no_mode();
        }
    }
}
