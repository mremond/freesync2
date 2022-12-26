use std::fs;
use uuid::Uuid;

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

// Read all files in a directory and return them if they are a file and text in the txt extension.
fn read_dir(path: &str) -> Vec<String> {
    println!("Reading files in directory: {}", path);
    let mut files: Vec<String> = Vec::new();

    if let Some(entries) = fs::read_dir(path).ok() {
        for entry in entries {
            if let Some(entry) = entry.ok() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name) = file_name.to_str() {
                            if file_name.ends_with(".txt") {
                                if let Ok(full_path) = path.canonicalize() {
                                    println!("Path: {:?}", full_path);
                                    files.push(full_path.to_str().unwrap().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Found {} files", files.len());
    files
}

#[allow(unused)]
struct Note {
    title: String,
    content: String
}

fn read_file(path: &str) -> Option<Note> {
    println!("Reading file: {}", path);

    // read file contents into a string
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    // break content by lines
    let lines: Vec<&str> = content.lines().collect();
    if let Some(first_line) = lines.first() {
        if first_line.starts_with("# ") {
            let t = first_line[2..].to_string();
            println!("Found a title: {}", t);

            return Some(Note{                
                title: t,
                content: lines[1..].join("\n")
            })
        }
    }

    let t = Uuid::new_v4().to_string();
    println!("No title found, generating a random one: {}", t);
    Some(Note{title: t, content: content})
}

// output a new file base on path, filename, and contents
fn write_file(path: &str, title: &str, contents: &str) {
    let full_path = format!("{}/{}.md", path, title);
    println!("Writing file: {}", full_path);

    match fs::write(full_path, contents) {
        Ok(_) => println!("File written successfully"),
        Err(err) => println!("Error: {}", err)
    };
}

fn main() {
    // get the command line arguments
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.as_slice() {
        [input, output] => {
            match (valid_dir(input), valid_dir(output)) {
                (Some(input), Some(_output)) => {
                    let files = read_dir(&input);
                    let notes = files.iter().map(|file| {
                        read_file(&file)
                    }).collect::<Vec<_>>();
                    for note in notes {
                        match note {
                            Some(note) => {
                                write_file(&output, &note.title, &note.content);
                            },
                            None => ()
                        }
                    }
                },
                _ => ()
            }
        },
        _ => {
            println!("Usage: ./freesync2 <input> <output>");
        }
    }
}