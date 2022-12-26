use std::fs;

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

    // get file name from path
    let file_name = 
        std::path::Path::new(path)
            .file_name().expect("Could not get file name")
            .to_str().expect("Could not convert to string.");
    let end = file_name.len() - ".txt".len();
    let t = file_name[..end].to_string();
    println!("No title found, using the original one: {}", t);
    Some(Note{title: t, content: content})
}

fn content_diff(old: &str, new: &str) -> Option<String> {
    if old.contains(new) {
        println!("Content already exists, skipping.");
        None
    } else if new.contains(old) {
        println!("New content contains the old content, replacing.");
        Some(new.to_string())
    } else {
        println!("New and old are different. Appending new content to the old.");
        Some(format!("{}\n{}", old, new))
    }
}

// output a new file base on path, filename, and contents
fn write_file(path: &str, title: &str, contents: &str) {
    // TODO: If the title has invalid characters, replace them with an underscore.
    let full_path = format!("{}{}.md", path, title);
    println!("Writing file: {}", full_path);

    // check if the file already exists and append if so.
    if std::path::Path::new(&full_path).exists() {
        let old = fs::read_to_string(&full_path).expect("Something went wrong reading the file");
        match content_diff(&old, contents) {
            Some(contents) => {
                match fs::write(full_path, contents) {
                    Ok(_) => println!("File written successfully"),
                    Err(err) => println!("Error: {}", err)
                };
            },
            None => ()
        }
    } else {
        match fs::write(full_path, contents) {
            Ok(_) => println!("File written successfully"),
            Err(err) => println!("Error: {}", err)
        };
    }
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