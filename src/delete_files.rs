use std::{path::PathBuf, time::Instant, fs, io};
use filesize::file_real_size;

pub fn delete_files(index: (Vec<PathBuf>, usize)) {
    let mut total_file_size = 0;

    // iterates through filtered files, adds size to total, then deletes them
    let now = Instant::now();
    for file in index.0.iter() {
        total_file_size += file_real_size(file).unwrap();
        if !file.as_path().to_str().unwrap().contains("song") {
            fs::remove_file(file).unwrap();
        }
    }
    let elapsed_time = now.elapsed();
    println!("\n{} files deleted totaling {}mb in {} seconds", index.1, total_file_size/1000000, elapsed_time.as_secs());
    
    // idk how to make the program not just immediately close itself
    println!("\nPress a key to exit.");
    let mut wait = String::new();
    io::stdin()
        .read_line(&mut wait)
        .expect("Invalid input.");
}
