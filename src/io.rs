use crate::obs;
use crate::diff::Diffable;
use std::fs;

pub struct Dir {
    path: String
}

impl Dir {
    pub fn new_valid(path: &str) -> Option<Dir> {
        match valid_dir(path) {
            Some(path) => Some(Dir{path: path}),
            None => None
        }
    }
    pub fn put_note(&self, note: &obs::Note) -> Result<Option<String>, std::io::Error> {
        let full_path = if self.path.ends_with("/") {
            format!("{}{}.md", self.path, note.title)
        } else {
            format!("{}/{}.md", self.path, note.title)
        };
        println!("Writing file: {}", full_path);
        
        write_file(&full_path, &note.title, &note.content);
    }
    pub fn list_text_files(&self) -> Vec<String> {
        read_dir(&self.path)
    }
}

// Take a path and check if it points to a valid directory.
fn valid_dir(path: &str) -> Option<String> {
    println!("Checking path: {}", path);
    let metadata = fs::metadata(path);
    match metadata {
        Ok(metadata) => {
            if metadata.is_dir() {
                println!("Path is a directory");
                Some(path.to_string())
            } else {
                println!("Path is not a directory");
                None
            }
        },
        Err(err) => {
            println!("Error: {}", err);
            None
        }
    }
}

// Test known directories are valid.
#[test]
fn test_valid_dir() {
    assert_eq!(valid_dir("/"), Some("/".to_string()));
    assert_eq!(valid_dir("src"), Some("src".to_string()));
    assert_eq!(valid_dir("/tmp"), Some("/tmp".to_string()));
    assert_eq!(valid_dir("resources"), Some("resources".to_string()));
}

#[test]
fn test_files_not_dir() {
    assert_eq!(valid_dir("Cargo.toml"), None);
    assert_eq!(valid_dir("src/main.rs"), None);
    assert_eq!(valid_dir("resources/hello_world.txt"), None);
}

#[test]
fn test_missing_dir() {
    assert_eq!(valid_dir("/missing"), None);
    assert_eq!(valid_dir("missing"), None);
}

// Read all files in a directory and return them if they are a file and text in the txt extension.
fn read_dir(path: &str) -> Vec<String> {
    println!("Reading files in directory: {}", path);
    let mut files: Vec<String> = Vec::new();

    for entry in fs::read_dir(path).expect("Could not read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            println!("Evaluating path: {:?}", path);
            if path.is_file() {
                let file_name = 
                    path.file_name().expect("Could not get file name")
                        .to_str().expect("Could not convert to string.");

                if file_name.ends_with(".txt") {
                    let full_path = path.canonicalize().expect("Could not get full path");
                    println!("Path: {:?}", full_path);
                    files.push(full_path.to_str().unwrap().to_string());
                } else {
                    println!("Path is not a text file.");
                }
            } else {
                println!("Path is not a file.");
            }
        }
    }

    println!("Found {} files", files.len());
    files
}

// Output a new file or append to an existing one. Returns the title of the
// file if it was written successfully for use as an alias.
fn write_file(full_path: &str, title: &str, content: &str) -> Result<Option<String>, std::io::Error> {
// check if the file already exists and append if so.
    if std::path::Path::new(&full_path).is_file() {
        let old = fs::read_to_string(&full_path).expect("Something went wrong reading the file");
        match old.diff(content) {
            Some(contents) => {
                match fs::write(full_path, contents) {
                    Ok(_) => {
                        println!("File written successfully");
                        return Ok(Some(title.to_owned()))
                    },
                    Err(err) => return Err(err)
                };
            },
            None => Ok(None)
        }
    } else {
        match fs::write(full_path, content) {
            Ok(_) => {
                println!("New file written successfully");
                return Ok(Some(title.to_owned()));
            },
            Err(err) => return Err(err)
        };
    }    
}

pub struct File {
    full_path: String
}

impl File {
    pub fn new_valid(path: &str) -> Option<File> {
        match valid_file(&path) {
            Some(path) => Some(File{full_path: path}),
            None => None
        }
    }
    pub fn read_note(&self) -> Option<obs::Note> {
        read_file(&self.full_path.to_owned())
    }
    pub fn put_content(&self, content: &str) {
        append_to_file(&self.full_path, content)
    }
}


fn append_to_file(full_path: &str, contents: &str) {
    if std::path::Path::new(&full_path).exists() {
        let old = fs::read_to_string(&full_path).expect("Something went wrong reading the file");
        match old.diff(&contents) {
            Some(contents) => {
                match fs::write(full_path, contents) {
                    Ok(_) => println!("File written successfully"),
                    Err(err) => println!("Error: {}", err)
                };
            },
            None => ()
        }
    } else {
        println!("File expected to exists for append: {}", full_path);
    }
}

fn read_file(path: &str) -> Option<obs::Note> {
    println!("Reading file: {}", path);

    // read file contents into a string
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    match obs::Note::parse(&content) {
        Some(note) => {
            println!("Found a title in the content.");
            return Some(note);
        },
        // Default titles caused issues, so only create a note if properly formated for a title.
        None => {
            println!("No title found in content for file: {}", path);
            None
        }
    } 
}

fn valid_file(path: &str) -> Option<String> {
    println!("Checking path: {}", path);
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("Path is a file");
                Some(path.to_string())
            } else {
                println!("Path is not a file");
                None
            }
        },
        Err(err) => {
            println!("Error: {}", err);
            None
        }
    }
}

#[test]
fn test_valid_file() {
    assert_eq!(valid_file("Cargo.toml"), Some("Cargo.toml".to_string()));
    assert_eq!(valid_file("src/main.rs"), Some("src/main.rs".to_string()));
    assert_eq!(valid_file("resources/hello_world.txt"), Some("resources/hello_world.txt".to_string()));
}

#[test]
fn test_dirs_not_file() {
    assert_eq!(valid_file("/"), None);
    assert_eq!(valid_file("src"), None);
    assert_eq!(valid_file("/tmp"), None);
    assert_eq!(valid_file("resources"), None);
}

#[test]
fn test_missing_file() {
    assert_eq!(valid_file("/missing.txt"), None);
    assert_eq!(valid_file("missing.txt"), None);
}
