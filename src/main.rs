mod commands;
mod utilities;
use crate::utilities::{Result, terminal};
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "SoulsCLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(alias = "p")]
    Publish {
        #[arg(default_value = "")]
        crate_name: String,
        #[arg(short = 'd', long)]
        dry_run: bool,
    },
    #[command(alias = "c")]
    Cargo {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(alias = "g")]
    Git {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(alias = "n")]
    New { project_name: String },
    #[command(alias = "m")]
    Menu,
    #[command(alias = "t")]
    Test,
}

impl Command {
    fn dispatch(self) -> Result<()> {
        match self {
            Self::Publish {
                crate_name,
                dry_run,
            } => {
                let mut args = vec!["publish"];
                if !crate_name.is_empty() {
                    args.extend(["-p", &crate_name]);
                }
                if dry_run {
                    args.push("--dry-run");
                }
                terminal("cargo", &args)?;
            }
            Self::Cargo { args } => run_tool("cargo", &args)?,
            Self::Git { args } => run_tool("git", &args)?,
            Self::New { project_name } => commands::new::run(&project_name)?,
            Self::Menu => commands::menu::run()?,
            Self::Test => commands::menu::souls::test()?,
        }
        Ok(())
    }
}

fn run_tool(tool: &str, args: &[String]) -> Result<()> {
    let args: Vec<&str> = args.iter().map(std::string::String::as_str).collect();
    terminal(tool, &args)
}

fn main() {
    if let Err(e) = run() {
        let _ = cliclack::log::error(format!("{e}"));
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    Cli::parse().command.dispatch()
}
