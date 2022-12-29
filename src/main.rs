mod io;
mod obs;

#[derive(PartialEq, Debug)]
struct Arguments {
    input: String,
    output: String,
    alias: Option<String>
}

fn args_to_arguments(args: Vec<String>) -> Option<Arguments> {
    match args.as_slice() {
        [input, output] => {
            match (io::valid_dir(input), io::valid_dir(output)) {
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
            match (io::valid_dir(input), io::valid_dir(output), io::valid_file(alias)) {
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
    let args = vec!["src".to_string(), "resources".to_string(), "resources/alias.md".to_string()];
    assert_eq!(args_to_arguments(args), Some(Arguments{input: "src".to_string(), output: "resources".to_string(), alias: Some("resources/alias.md".to_string())}));
}

fn move_files(input: &str, output: &str) -> Vec<String> {
    // Read the input directory and iterate over the text files within it.
    io::read_dir(&input).iter()
    // Convert the file to a note. 
    .map(|file| {
        io::read_file(&file)
    })
    // If a valid note was found, then write it to the output directory.
    .map(|note| {
        match note {
            Some(note) => {
                match note.write_file(&output) {
                    Some(alias) => return alias,
                    None => ()
                };
            },
            None => ()
        };
        "".to_string()
    })
    .filter(|alias| alias != "")
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
                    match io::valid_file(&alias) {
                        Some(_) => {
                            move_files(&input, &output).iter()
                            .for_each(|title| {
                                println!("Appending to alias file: {}", title);
                                let block = format!("----\n\n![[{}]]\n\n", title);
                                io::append_to_file(&alias, &block);
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