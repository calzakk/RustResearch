use std::io::{Read, Write};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Split {
        /// File pathname
        pathname: String,

        /// Number of parts to split the file into
        count: usize
    },
    Combine {
        /// File pathname
        pathname: String,

        /// Don't delete split files after combining
        #[arg(short, long)]
        no_cleanup: bool
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Split { pathname, count } => {
            split(pathname.as_str(), count);
        },
        Commands::Combine { pathname, no_cleanup } => {
            combine(pathname.as_str(), no_cleanup);
        }
    }
}

fn split(pathname: &str, count: usize) {
    if count < 2 {
        println!("Count must be greater than 1");
        return;
    }

    let metadata = std::fs::metadata(pathname).unwrap();
    let file_size = metadata.len() as usize;
    let part_size = (file_size + count - 1) / count;
    let min_part_size = 1024;
    if part_size < min_part_size {
        println!("Count is too high, part size is only {}", part_size);
        return;
    }

    let part_filename = format!("{}.0", pathname);
    if std::fs::metadata(part_filename).is_ok() {
        println!("File has already been split");
        return;
    }

    let mut file = std::fs::File::open(pathname).unwrap();

    println!("Splitting {} into {} parts", pathname, count);

    let mut total_bytes_read = 0;
    let mut total_bytes_written = 0;

    for part in 0..count {
        let part_pathname = format!("{}.{}", pathname, part);
        println!("{}", part_pathname);

        let mut part_file = std::fs::File::create(part_pathname).unwrap();
        let max_buffer_size = 1024 * 1024;
        let mut remaining_part_size = part_size;

        while remaining_part_size > 0 {
            let buffer_size = std::cmp::min(remaining_part_size, max_buffer_size);
            let mut buffer = vec![0; buffer_size];

            let bytes_read: usize = file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            remaining_part_size -= bytes_read;
            total_bytes_read += bytes_read;

            part_file.write_all(&buffer[..bytes_read]).unwrap();
            total_bytes_written += bytes_read;
        }
    }

    if total_bytes_read != file_size {
        println!("Error reading file - bytes read: {}, file size: {}", total_bytes_read, file_size);
        return;
    }

    if total_bytes_written != file_size {
        println!("Error writing file - bytes written: {}, file size: {}", total_bytes_written, file_size);
        return;
    }

    println!("Complete");
}

fn combine(pathname: &str, no_cleanup: bool) {
    if std::fs::metadata(pathname).is_ok() {
        println!("Output file already exists");
        return;
    }

    let mut next_part = 0;
    loop {
        let part_filename = format!("{}.{}", pathname, next_part);
        if !std::fs::metadata(part_filename).is_ok() {
            break;
        }
        next_part += 1;
    }

    if next_part == 0 {
        println!("No parts found");
        return;
    }

    let last_part = next_part - 1;

    for missing in 1..10 {
        let missing_part_filename = format!("{}.{}", pathname, last_part + missing);
        if std::fs::metadata(&missing_part_filename).is_ok() {
            for i in 1..missing {
                println!("Part {} is missing", last_part + i);
            }
            return;
        }
    }

    println!("Combining parts {}..{} into {}", 0, last_part, pathname);

    let mut file = std::fs::File::create(pathname).unwrap();

    let mut total_bytes_read = 0;
    let mut total_bytes_written = 0;

    for part in 0..next_part {
        let part_filename = format!("{}.{}", pathname, part);
        let mut part_file = std::fs::File::open(part_filename).unwrap();
        let mut buffer = vec![0; 1024 * 1024];

        loop {
            let bytes_read: usize = part_file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            total_bytes_read += bytes_read;

            file.write_all(&buffer[..bytes_read]).unwrap();
            total_bytes_written += bytes_read;
        }
    }

    if total_bytes_written != total_bytes_read {
        println!("ERROR: Bytes read: {}, bytes written: {}", total_bytes_read, total_bytes_written);
        println!("No cleanup")
    }
    else {
        println!("Bytes read and written: {}", total_bytes_written);

        if !no_cleanup {
            println!("Deleting split files");
            for part in 0..last_part + 1 {
                let part_filename = format!("{}.{}", pathname, part);
                std::fs::remove_file(part_filename).unwrap();
            }
        }

        println!("Complete");
    }
}