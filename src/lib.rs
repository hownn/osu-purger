use filesize::file_real_size;
use walkdir::{DirEntry,WalkDir};
use std::{path::PathBuf, fs, io};

pub fn get_path() -> String {
    let mut path = String::new();

    println!("Input the path to your osu! song directory:");
    // user inputs path to song directory
    io::stdin() 
        .read_line(&mut path)
        .expect("Invalid path.");
    // checks if path containts osu!\Songs as that's the standard directory
    if path.as_str().contains("osu!\\Songs") == false {
        println!("'{}' doesn't contain \"osu!\\Songs\", are you sure this is the correct path?\ny/n", path.trim());
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Invalid response");

        // panics if user indicates it's the wrong path or does an invalid input
        match response.as_str().trim() {
            "y" => path,
            "n" => panic!("Wrong path"),
            _ => panic!("Invalid input"),
        }
    } else {
        path
    }
}
    
pub fn index_files(path: String) -> Vec<DirEntry> {
    let mut indexed_files = Vec::new();
    for entry in WalkDir::new(path.trim()).into_iter().filter_map(|e| e.ok()) {
        indexed_files.push(entry);
    }
    // asks the user for confirmation they want to delete the files
    println!("Found {} files, do you want to continue?\ny/n", indexed_files.len()-1);
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

pub fn filter_files(files: Vec<DirEntry>) -> Vec<PathBuf> {
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

pub fn delete_files(filtered_files: Vec<PathBuf>) {
    // amount of files = length of our vector
    let file_amount = filtered_files.len();
    let mut total_file_size = 0;

    // iterates through filtered files, adds size to total, then deletes them
    for file in filtered_files.iter() {
        total_file_size += file_real_size(file).unwrap();
        fs::remove_file(file).unwrap();
    }
    println!("{} files deleted totaling {}mb", file_amount, total_file_size/1000000);

    // idk how to make the program not just immediately close itself
    let mut useless = String::new();
    io::stdin()
        .read_line(&mut useless)
        .expect("Invalid input.");
}

