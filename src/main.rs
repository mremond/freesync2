use std::fs::File;

// main function
fn main() {
    print_file_or_dir("src");
}

fn get_file(filename: &str) -> Option<File> {
    let file = File::open(filename);
    match file {
        Ok(file) => Some(file),
        Err(error) => {
            println!("Error: {}", error);
            None
        }
    }
}

// create an enum for file, directory, or neither
enum FileType {
    File,
    Directory,
    Neither,
}

// get the file type
fn get_file_type(filename: &str) -> FileType {
    let file = get_file(filename);
    match file {
        Some(file) => {
            let metadata = file.metadata().unwrap();
            if metadata.is_file() {
                FileType::File
            } else if metadata.is_dir() {
                FileType::Directory
            } else {
                FileType::Neither
            }
        },
        None => FileType::Neither
    }
}

// if optional file then match the file_type 
fn print_file_or_dir(filename: &str) {
    let file_type = get_file_type(filename);
    match file_type {
        FileType::File => println!("File: {}", filename),
        FileType::Directory => println!("Directory: {}", filename),
        FileType::Neither => println!("Neither: {}", filename),
    }
}