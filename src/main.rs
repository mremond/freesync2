use std::fs;

/// Take a path and check if it points to a valid directory.
fn valid_dir(path: &str) -> Option<String> {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(metadata) => {
            if metadata.is_dir() {
                Some(path.to_string())
            } else {
                None
            }
        },
        Err(_) => {
            None
        }
    }
}

/**
 * Main function
 *
 * 1. Parse the command line arguments
 * 2. Decide whether to process a file or a directory
 * 3. Decide whether to output to a file or a directory
 * 4. Run the process based on input/output types.
 */
fn main() {
    // get the command line arguments
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.as_slice() {
        [input, output] => {
            match (valid_dir(input), valid_dir(output)) {
                (Some(input), Some(output)) => {
                    println!("Processing directory {} to directory {}", input, output);
                },
                _ => ()
            }
        },
        _ => {
            println!("Usage: ./freesync2 <input> <output>");
        }
    }
}