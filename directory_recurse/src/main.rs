use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;

fn main() {
    recurse(Path::new("."));
}

fn recurse(path: &Path) {
    println!("\n[{:?}]", path);
    match fs::read_dir(path) {
        Ok(entries) => {
            let sorted = sort_entries(entries);
            print_entries(&sorted);

            for dir in sorted {
                if dir.metadata().unwrap().is_dir() {
                    recurse(&dir.path())
                }
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }
}

fn sort_entries(entries: ReadDir) -> Vec<DirEntry> {
    let mut sorted: Vec<DirEntry> = Vec::new();
    for entry in entries {
        let dir = entry.unwrap();
        let index = find_index(&sorted, &dir);
        sorted.insert(index, dir);
    }
    sorted
}

fn find_index(v: &Vec<DirEntry>, dir: &DirEntry) -> usize {
    let mut index = 0;
    for vdir in v {
        if dir.file_name() < vdir.file_name() {
            break;
        }
        index += 1;
    }
    index
}

fn print_entries(v: &Vec<DirEntry>) {
    for dir in v {
        println!("{:?}", dir.file_name());
    }
}
