use std::fs;

/// Given a path, return Some(File) if it's a directory; None if it's not.
/// 
///     assert_eq!(get_dir("src"), Some(File));
pub fn get_dir(path: &str) -> Option<fs::File> {
    let dir = fs::File::open(path);
    match dir {
        Ok(dir) => {
            // make sure it's a directory
            let metadata = dir.metadata().unwrap();
            if metadata.is_dir() {
                Some(dir)
            } else {
                println!("Error: {} is not a directory", path);
                None
            }
        }
        Err(error) => {
            println!("Error accessing path: {}", error);
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
// parse the command line argument
    let args: Vec<String> = std::env::args().skip(1).collect();

    let _files = args.iter().map(|arg| get_dir(arg)).collect::<Vec<_>>();
}