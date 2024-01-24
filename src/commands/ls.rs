use clap::Args;
use std::{env::current_dir, ffi::OsString, os::unix::fs::PermissionsExt};
#[allow(unused_imports)]
use std::fs::{self, DirEntry};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct LsArgs {
    pub path: Option<PathBuf>,

    #[arg(short, long, help = "show all files, including hidden ones")]
    pub all: bool,

    #[arg(short, help = "use a long listing format")]
    pub long: bool,
}

impl LsArgs {
    pub fn run(&self) {
        let work_dir = current_dir().unwrap(); // working directory
        let target_dir = self.path.as_ref().unwrap_or(&work_dir); // if path option is not given, set working directory to target_dir
        let mut item_list: Vec<FileMetadata> = vec![];
        match fs::read_dir(target_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            // println!("{:#?}", entry.file_name());
                            item_list.push(FileMetadata {name: entry.file_name(), metadata: entry.metadata().unwrap()});
                        }
                        Err(e) => eprintln!("error: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("error: {}", e),
        }
        // println!("size of item_list: {}", item_list.len());

        if self.long {
            for item in item_list {
                println!("{:?} {} {:?}", item.metadata.permissions().mode(), item.metadata.len(), item.name);
            }
        }
    }
}

struct FileMetadata {
    name: OsString,
    metadata: fs::Metadata,
}
