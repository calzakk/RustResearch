use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: {} [path]", args[0]);
        return;
    }

    let path = match args.len() {
        1 => Path::new("."),
        _ => Path::new(&args[1])
    };

    let mut dirs_to_remove = Vec::new();
    recurse(path, &mut dirs_to_remove);

    if dirs_to_remove.is_empty() {
        println!("Nothing found!");
        return;
    }

    list_dirs(&dirs_to_remove);

    println!("Do you want to remove the directories? [y/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_uppercase() != "Y" {
        println!("Aborted!");
        return;
    }

    remove_dirs(&dirs_to_remove);
}

fn recurse(path: &Path, dirs_to_remove: &mut Vec<DirEntry>) {
    match fs::read_dir(path) {
        Ok(entries) => {
            let dirs = find_dirs(entries);

            for dir in dirs {
                let path = dir.path();

                if path.ends_with("target/debug") || path.ends_with("target/release") {
                    dirs_to_remove.push(dir);
                    continue;
                }

                if dir.metadata().unwrap().is_dir() {
                    recurse(&dir.path(), dirs_to_remove);
                }
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }
}

fn find_dirs(entries: ReadDir) -> Vec<DirEntry> {
    let mut sorted: Vec<DirEntry> = Vec::new();
    for entry in entries {
        let dir = entry.unwrap();
        if dir.metadata().unwrap().is_dir() {
            sorted.push(dir);
        }
    }
    sorted
}

fn list_dirs(dirs: &Vec<DirEntry>) {
    println!("Found {} folder{} to remove:", dirs.len(), if dirs.len() > 1 { "s" } else { "" });
    for dir in dirs {
        println!("{:?}", dir.path());
    }
}

fn remove_dirs(dirs: &Vec<DirEntry>) {
    for dir in dirs {
         fs::remove_dir_all(dir.path()).unwrap();
    }
}