use filesize::file_real_size;
use walkdir::{DirEntry,WalkDir};
use std::{path::PathBuf, fs, io};

fn main() {
    let mut path = String::new();
    let mut indexed_files = Vec::new();

    println!("Input the path to your osu! song directory:");
    // user inputs path to song directory
    io::stdin() 
        .read_line(&mut path)
        .expect("Invalid path.");
    // pushes all files in the directory to a vector
    for entry in WalkDir::new(path.trim()).into_iter().filter_map(|e| e.ok()) { 
        indexed_files.push(entry);
    }
    // forwards vector to filter files function
    let filtered_files = filter_files(indexed_files);
    
    // forwards vector to delete files function
    delete_files(filtered_files);
}

fn filter_files(files: Vec<DirEntry>) -> Vec<PathBuf> {
    // vector with extensions we want to filter out
    let wanted_extensions = vec!["wav", "mp4", "jpg", "png", "mkv", "flv", "jpeg", "ogg", "avi", "wmv", "mpg", "mov", "m4v", "mpeg", "3gp", "webm", "webp", "bmp", "heif", "svg", "aac"];
    let mut filtered_files = Vec::new();

    // iterates through indexed files then pushes them to vector if extension matches
    for file in files.iter() {
        let extension = file.path().extension();
        if extension.map_or(false, |ext| wanted_extensions.iter().any(|&x| x == ext)) {
            filtered_files.push(file.path().to_owned());
        }
    }
    filtered_files
}

fn delete_files(filtered_files: Vec<PathBuf>) {
    // amount of files = length of our vector
    let file_amount = filtered_files.len();
    let mut total_file_size = 0;

    // iterates through filtered files, adds size to total, then deletes them
    for file in filtered_files.iter() {
        total_file_size += file_real_size(file).unwrap();
        delete_file(file);
    }
    println!("{} files deleted totaling {} mb", file_amount, total_file_size/1000000);

    // idk how to make the program not just immediately close itself
    let mut useless = String::new();
    io::stdin()
        .read_line(&mut useless)
        .expect("Invalid input.");
}

fn delete_file(path: &PathBuf) {
    // just deletes the files
    fs::remove_file(path).unwrap();
}
