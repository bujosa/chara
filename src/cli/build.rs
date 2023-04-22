use clap::{command, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command()]
pub struct BuildCommand {
    #[arg(long)]
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
    #[arg(long, help = "Truly delete the build")]
    truly: Option<bool>,

    /// Indicate if the build should be deleted
    /// from the local machine
    /// or from the remote machine
    #[arg(long)]
    local: Option<bool>,
}

pub fn parse(sub_command: BuildCommand) {
    let res = match sub_command {
        BuildCommand {
            active_build: Some(true),
            command: BuildSubCommand::Delete(args),
        } => {
            let res = match args {
                GetDeleteArgs {
                    truly: Some(true), ..
                } => "Truly delete",
                GetDeleteArgs {
                    local: Some(true), ..
                } => "Delete from local",
                _ => "Delete",
            };
            res
        }
        _ => "No mode",
    };

    println!("{}", res);
}
