use walkdir::DirEntry;
use std::path::PathBuf;

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
