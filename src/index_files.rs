use walkdir::{WalkDir};
use std::{io, path::PathBuf};

pub fn index_files(path: String) -> (Vec<PathBuf>, usize) {
    let mut indexed_files = Vec::new();
    let wanted_extensions = vec!["wav", "mp4", "jpg", "png", "mkv", "jpeg", "flv", "ogg", "wmv", "mov", "m4v", "WAV", "MP4", "JPG", "PNG", "MKV", "JPEG", "FLV", "OGG", "WMV", "MOV", "M4V:"];

    for entry in WalkDir::new(path.trim()).into_iter().filter_map(|e| e.ok()) {
        let extension = entry.path().extension();
        if extension.map_or(false, |ext| wanted_extensions.iter().any(|&x| x == ext)) {
            indexed_files.push(entry.path().to_owned())
        }
    }
    let amount_of_files = indexed_files.len();

    // asks the user for confirmation they want to delete the files
    println!("\nFound directory {} with {} files, do you want to continue?\ny/n", path.trim(), amount_of_files);
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Invalid response");

    // panics if user indicates they don't want to delete the files
    match response.as_str().trim() {
        "y" => (indexed_files, amount_of_files),
        "n" => panic!("Cancelled deletion"),
        _ => panic!("Invalid input"),
    }
}
