use sha2::{Sha256, Digest};
use std::{fs::File, io::{BufReader, Read}};

#[derive(Debug)]
pub enum FileType {
    File,
    Directory,
    Neither,
}

impl PartialEq for FileType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileType::File, FileType::File) => true,
            (FileType::Directory, FileType::Directory) => true,
            (FileType::Neither, FileType::Neither) => true,
            _ => false
        }
    }
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

// get the file type
pub fn get_file_type(filename: &str) -> FileType {
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

pub fn read_file_contents(filename: &str) -> Option<String> {
    let file = get_file(filename);
    match file {
        Some(file) => {
            let mut contents = String::new();
            let mut buf = BufReader::new(file);
            buf.read_to_string(&mut contents).unwrap();
            Some(contents)
        },
        None => None
    }
}

fn get_hash(contents: &str) -> String {    
    let mut hasher = Sha256::new();
    hasher.update(contents);
    format!("{:x}", hasher.finalize())
}

pub fn file_hash(filename: &str) -> Result<String, &'static str> {
    match get_file_type(filename) {
        FileType::File => {
            let contents = read_file_contents(filename).unwrap();
            Ok(get_hash(&contents))
        },
        FileType::Directory => Err("Cannot hash a directory"),
        FileType::Neither => Err("Cannot hash a non-existent file"),
    }
}

#[test]
fn test_get_file_type() {
    assert_ne!(get_file_type("src/main.rs"), FileType::Neither);
    assert_ne!(get_file_type("src/main.rs"), FileType::Directory);
    assert_eq!(get_file_type("src/main.rs"), FileType::File);
}

#[test]
fn test_get_file_type_directory() {
    assert_ne!(get_file_type("src"), FileType::Neither);
    assert_ne!(get_file_type("src"), FileType::File);
    assert_eq!(get_file_type("src"), FileType::Directory);
}

#[test]
fn test_read_hello_world() {
    assert_eq!(read_file_contents("resources/hello_world.txt").unwrap(), "Hello World!");
}

#[test]
fn test_hash_hello_world() {
    assert_eq!(get_hash("Hello World!"), "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069");
}

#[test]
fn test_hash_hello_world_file() {
    assert_eq!(file_hash("resources/hello_world.txt").unwrap(), "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069");
}