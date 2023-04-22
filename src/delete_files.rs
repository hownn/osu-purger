use std::{path::PathBuf, fs, io};
use filesize::file_real_size;

pub fn delete_files(filtered_files: Vec<PathBuf>) {
    // amount of files = length of our vector
    let file_amount = filtered_files.len();
    let mut total_file_size = 0;

    // iterates through filtered files, adds size to total, then deletes them
    for file in filtered_files.iter() {
        total_file_size += file_real_size(file).unwrap();
        fs::remove_file(file).unwrap();
    }
    println!("\n{} files deleted totaling {}mb", file_amount, total_file_size/1000000);
    
    // idk how to make the program not just immediately close itself
    let mut useless = String::new();
    io::stdin()
        .read_line(&mut useless)
        .expect("Invalid input.");
}
