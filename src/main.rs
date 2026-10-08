mod commands;
mod utilities;
use crate::utilities::terminal;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "s", version = "0.0.1")]
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
}

impl Command {
    fn dispatch(self) {
        match self {
            Command::Publish { crate_name, dry_run } => {
                let mut args = vec!["publish"];
                if !crate_name.is_empty() { args.extend(["-p", &crate_name]); }
                if dry_run { args.push("--dry-run"); }
                terminal("cargo", &args);
            }
            Command::Cargo { args } => run_tool("cargo", &args),
            Command::Git { args } => run_tool("git", &args),
            Command::New { project_name } => commands::new::run(&project_name),
            Command::Menu => commands::menu::run(),
        }
    }
}

fn run_tool(tool: &str, args: &[String]) {
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    terminal(tool, &args);
}

fn main() { Cli::parse().command.dispatch() }