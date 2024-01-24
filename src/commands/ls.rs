use std::path::PathBuf;
use std::env::current_dir;
use std::fs;
use clap::Args;


#[derive(Args, Debug)]
pub struct LsArgs {
    pub path: Option<PathBuf>,

    #[arg(short, long, help = "show all files, including hidden ones")]
    pub all: bool,

    #[arg(short, help = "use a long listing format")]
    pub long: bool
}

impl LsArgs {
    pub fn run(&self) {
        let work_dir = current_dir().unwrap(); // working directory
        let target_dir = self.path.as_ref().unwrap_or(&work_dir); // if path option is not given, set working directory to target_dir
        
        if self.all {
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
        } else {
            println!("no any options");
        }
    }
}
