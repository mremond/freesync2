use sha2::{Sha256, Digest};

mod file_utils;

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
    let _args: Vec<String> = std::env::args().skip(1).collect();

    match file_utils::get_file_type("src/main.rs") {
        file_utils::FileType::File => {
            let contents = file_utils::read_file_contents("src/main.rs").unwrap();
            println!("{}", get_hash(&contents));
        },
        file_utils::FileType::Directory => println!("Directory"),
        file_utils::FileType::Neither => println!("Neither"),
    }
}

fn get_hash(contents: &str) -> String {    
    let mut hasher = Sha256::new();
    hasher.update(contents);
    format!("{:x}", hasher.finalize())
}

// // structure with origin path, output path, filename, origin hash and output hash
// struct Metadata {
//     origin_path: String,
//     output_path: String,
//     filename: String,
//     origin_hash: String,
//     output_hash: String,
// }

// // structure with metadata and contents, both mutable
// struct ProcessData {
//     metadata: Metadata,
//     contents: String,
// }
