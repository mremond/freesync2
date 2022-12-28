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

#[derive(PartialEq, Debug)]
struct Note {
    title: String,
    content: String
}

fn find_content_title(content: &str) -> Option<Note> {
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
    None
}

#[test]
fn test_blank_content() {
    assert_eq!(find_content_title(""), None);
}

#[test]
fn test_no_title() {
    assert_eq!(find_content_title("Hello World"), None);
}

#[test]
fn test_title() {
    assert_eq!(find_content_title("# My Title"), Some(Note{title: "My Title".to_string(), content: "".to_string()}));
    assert_eq!(find_content_title("# My Title\nAnd my content."), Some(Note{title: "My Title".to_string(), content: "And my content.".to_string()}));
}

#[test]
fn test_header_two() {
    assert_eq!(find_content_title("## My Title"), None);
}

#[test]
fn test_hash_tag() {
    assert_eq!(find_content_title("#My Title"), None);
}

fn read_file(path: &str) -> Option<Note> {
    println!("Reading file: {}", path);

    // read file contents into a string
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");

    match find_content_title(&content) {
        Some(note) => {
            println!("Found a title in the content: {}", note.title);
            return Some(note);
        },
        None => {
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
    } 
}

/// Compare content and handle cases where the file has already been moved before.
/// 
///     assert_eq!(content_diff("A", "A"), None);
///     assert_eq!(content_diff("A", "A\nB"), Some("A\nB".to_string()));
///     assert_eq!(content_diff("A\nB", "A"), Some("A\nB".to_string()));
///     assert_eq!(content_diff("A", "B"), Some("A\nB".to_string()));
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

#[test]
fn test_content_skip() {
    let old = "Hello World";
    let new = "Hello World";
    assert_eq!(content_diff(old, new), None);
}

#[test]
fn test_extra_content() {
    let old = "Hello World";
    let new = "Hello World\nThis is a new line";
    assert_eq!(content_diff(old, new), Some(new.to_string()));
}

#[test]
fn test_appended_content() {
    let old = "Hello World";
    let new = "This is totally new content!";
    assert_eq!(content_diff(old, new), Some(format!("{}\n{}", old, new)));
}

/// Remove the characters that Obsidian doesn't like in the title.
/// 
///     assert_eq!(check_title_chars("Hello World"), "Hello World");
///     assert_eq!(check_title_chars("Hello*World"), "Hello_World");
/// 
fn check_title_chars(title: &str) -> String {
    title.replace("*", "_")
         .replace("\"", "_")
         .replace("\\", "_")
         .replace("/", "_")
         .replace("<", "_")
         .replace("<", "_")
         .replace(">", "_")
         .replace(":", "_")
         .replace("|", "_")
         .replace("?", "_")
}

#[test]
fn test_check_title_chars() {
    // A couple strings that it should not change
    assert_eq!(check_title_chars("Hello World"), "Hello World");
    assert_eq!(check_title_chars("Hello_World"), "Hello_World");
    assert_eq!(check_title_chars("Hello-World"), "Hello-World");

    // All the strings that it should change.
    assert_eq!(check_title_chars("Hello*World"), "Hello_World");
    assert_eq!(check_title_chars("Hello\"World"), "Hello_World");
    assert_eq!(check_title_chars("Hello\\World"), "Hello_World");
    assert_eq!(check_title_chars("Hello/World"), "Hello_World");
    assert_eq!(check_title_chars("Hello<World"), "Hello_World");
    assert_eq!(check_title_chars("Hello>World"), "Hello_World");
    assert_eq!(check_title_chars("Hello:World"), "Hello_World");
    assert_eq!(check_title_chars("Hello|World"), "Hello_World");
    assert_eq!(check_title_chars("Hello?World"), "Hello_World");
}

// Output a new file base on path, filename, and contents Returns the 
// title of the file if it was written successfully for futher processing.
fn write_file(path: &str, title: &str, contents: &str) -> Option<String> {
    let title = check_title_chars(title);
    let full_path = format!("{}{}.md", path, title);
    println!("Writing file: {}", full_path);
    
    // check if the file already exists and append if so.
    if std::path::Path::new(&full_path).exists() {
        let old = fs::read_to_string(&full_path).expect("Something went wrong reading the file");
        match content_diff(&old, contents) {
            Some(contents) => {
                match fs::write(full_path, contents) {
                    Ok(_) => {
                        println!("File written successfully");
                        return Some(title);
                    },
                    Err(err) => println!("Error: {}", err)
                };
            },
            None => ()
        }
    } else {
        match fs::write(full_path, contents) {
            Ok(_) => {
                println!("File written successfully");
                return Some(title);
            },
            Err(err) => println!("Error: {}", err)
        };
    }
    None
}

fn append_to_file(full_path: &str, contents: &str) {
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
        println!("File expected to exists for append: {}", full_path);
    }
}

#[derive(PartialEq, Debug)]
struct Arguments {
    input: String,
    output: String,
    alias: Option<String>
}

fn args_to_arguments(args: Vec<String>) -> Option<Arguments> {
    match args.as_slice() {
        [input, output] => {
            match (valid_dir(input), valid_dir(output)) {
                (Some(_), Some(_)) => {
                    let ret = Arguments{input: input.to_string(), 
                                                   output: output.to_string(), 
                                                   alias: None};
                    Some(ret)
                },
                _ => None
            }
        },
        [input, output, alias] => {
            match (valid_dir(input), valid_dir(output), valid_file(alias)) {
                (Some(_), Some(_), Some(_)) => {
                    let ret = Arguments{input: input.to_string(), 
                                                   output: output.to_string(), 
                                                   alias: Some(alias.to_string())};
                    Some(ret)
                },
                _ => None
            }
        },
        _ => None
    }
}

#[test]
fn test_too_few_args() {
    let args = vec!["input".to_string()];
    assert_eq!(args_to_arguments(args), None);
}

#[test]
fn test_too_many_args() {
    let args = vec!["input".to_string(), "output".to_string(), "alias".to_string(), "extra".to_string()];
    assert_eq!(args_to_arguments(args), None);
}

#[test]
fn test_non_directory_args() {
    let args = vec!["input".to_string(), "output".to_string()];
    assert_eq!(args_to_arguments(args), None);
}

#[test]
fn test_directory_args() {
    let args = vec!["src".to_string(), "resources".to_string()];
    assert_eq!(args_to_arguments(args), Some(Arguments{input: "src".to_string(), output: "resources".to_string(), alias: None}));
}

#[test]
fn test_alias_args() {
    let args = vec!["src".to_string(), "resources".to_string(), "alias".to_string()];
    assert_eq!(args_to_arguments(args), Some(Arguments{input: "src".to_string(), output: "resources".to_string(), alias: Some("alias".to_string())}));
}

fn move_files(input: &str, output: &str) -> Vec<String> {
    // Read the input directory and iterate over the text files within it.
    read_dir(&input).iter()
    // Convert the file to a note. 
    .map(|file| {
        read_file(&file)
    })
    // If a valid note was found, then write it to the output directory.
    .map(|note| {
        match note {
            Some(note) => {
                match write_file(&output, &note.title, &note.content) {
                    Some(title) => return title,
                    None => ()
                };
            },
            None => ()
        };
        "".to_string()
    })
    .filter(|title| title != "")
    .collect()
}

fn main() {
    // interperet the command line arguments
    match args_to_arguments(std::env::args().skip(1).collect()) {
        // We found a simple job.
        Some(Arguments{input, output, alias}) => {
            match alias {
                None => {
                    let _ = move_files(&input, &output);
                },
                Some(alias) => {
                    match valid_file(&alias) {
                        Some(_) => {
                            move_files(&input, &output).iter()
                            .for_each(|title| {
                                println!("Appending to alias file: {}", title);
                                let block = format!("----\n\n![[{}]]\n\n", title);
                                append_to_file(&alias, &block);
                            });
                        },
                        None => ()
                    }
                }
            }
        },
        None => {
            println!("Usage: ./freesync2 <input> <output> [alias]");
            println!("input: must be a valid directory containing text files");
            println!("output: must be a valid directory");
            println!("alias: (if present) must be a valid file which already exists");
        }
    }
}