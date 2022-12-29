mod diff;
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
            match (io::Dir::new_valid(input), io::Dir::new_valid(output)) {
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
            match (io::Dir::new_valid(input), io::Dir::new_valid(output), io::File::new_valid(alias)) {
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
    let in_dir = io::Dir::new_valid(input).expect("Expct that the input directory is valid.");
    let out_dir = io::Dir::new_valid(output).expect("Expected that the output directory is valid.");

    // Read the input directory and iterate over the text files within it.
    in_dir.list_text_files().iter()
    // Convert the file to a note. 
    .map(|file| {
        io::File::new_valid(file).expect("Expected valid file.").read_note()
    })
    // If a valid note was found, then write it to the output directory.
    .map(|note| {
        match note {
            Some(note) => {
                match out_dir.put_note(&note) {
                    Ok(Some(alias)) => return alias,
                    Ok(None) => (),
                    Err(err) => {
                        println!("Error: {}", err);
                    }
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
        Some(Arguments{input, output, alias}) => {
            match alias {
                None => {
                    // No alias provided so do the simple move.
                    let _ = move_files(&input, &output);
                },
                Some(alias) => {
                    let file = io::File::new_valid(&alias).expect("Alias file should be valid.");

                    // Do the normal move, but this time iterate the aliases returned
                    move_files(&input, &output).iter()
                        .for_each(|title| {
                            // and put them as embeds in the alias file.
                            println!("Appending to alias file: {}", title);
                            let block = format!("----\n\n![[{}]]\n\n", title);
                            file.put_content(&block);
                        });
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