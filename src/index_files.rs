use walkdir::{WalkDir};
use std::{io, path::PathBuf};

pub fn index_files(path: String) -> (Vec<PathBuf>, usize) {
    let audio_extensions = vec!["wav", "mkv", "ogg"];
    let video_extensions = vec!["osb", "mp4", "mkv", "flv", "wmv", "mov", "m4v"];
    let image_extensions = vec!["jpg", "png", "jpeg"];

    let wanted_extensions = [audio_extensions, video_extensions, image_extensions].concat();
    let mut indexed_files = Vec::new();

    for entry in WalkDir::new(path.trim()).into_iter().filter_map(|e| e.ok()) {
        let extension = entry.path().extension();
        if extension.map_or(false, |ext| wanted_extensions.iter().any(|&x| x == ext.to_ascii_lowercase())) {
            indexed_files.push(entry.path().to_owned())
        }
    }
    let amount_of_files = indexed_files.len();

    // asks the user for confirmation they want to delete the files
    println!("\nFound directory {} with {} matching files, do you want to continue?\ny/n", path.trim(), amount_of_files);
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
