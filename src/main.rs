use std::path::PathBuf;
use std::env::current_dir;
#[allow(unused_imports)]
use std::fs;

use clap::{Parser, Subcommand};

// mod commands; // <-- This module may be used in the future to split all subcommands into separate files.

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
    Ls {
        path: Option<PathBuf>,
    },
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

        Commands::Ls { path } => {
            let work_dir = current_dir().unwrap(); // working directory
            let target_dir = path.unwrap_or(work_dir); // if path option is not given, set working directory to target_dir

            match fs::read_dir(target_dir) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(entry) => print!("{:?}  ", entry.file_name()),
                            Err(e) => eprintln!("error: {}", e),
                        }
                    }
                    println!();
                },
                Err(e) => eprintln!("error: {}", e),
            }
        }
    }
}
