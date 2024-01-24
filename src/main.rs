// use std::path::PathBuf;
use std::env::current_dir;
#[allow(unused_imports)]
use std::fs;

use clap::{Parser, Subcommand};

mod commands;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = false)]
    Pwd,

    #[command(arg_required_else_help = false)]
    Ls(commands::ls::LsArgs),
    // Test {
    //     /// lists test values
    //     #[arg(short, long)]
    //     list: bool,
    // },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Pwd => {
            println!("{}", current_dir().unwrap().to_str().unwrap());
        },

        Commands::Ls(ls_args) => {
            ls_args.run();
        }
    }
}
