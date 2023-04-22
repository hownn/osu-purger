use walkdir::{WalkDir, DirEntry};
use std::io;

pub fn index_files(path: String) -> Vec<DirEntry> {
    let mut indexed_files = Vec::new();
    for entry in WalkDir::new(path.trim()).into_iter().filter_map(|e| e.ok()) {
        indexed_files.push(entry);
    }
    // asks the user for confirmation they want to delete the files
    println!("\nFound {} files, do you want to continue?\ny/n", indexed_files.len()-1);
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Invalid response");

    // panics if user indicates they don't want to delete the files
    match response.as_str().trim() {
        "y" => indexed_files,
        "n" => panic!("Cancelled deletion"),
        _ => panic!("Invalid input"),
    }
}
