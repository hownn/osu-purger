use std::{io, path::PathBuf, time::Instant};
use walkdir::WalkDir;

pub fn index_files(path: String) -> (Vec<PathBuf>, usize) {
    // defines the file extensions associated with each filetype
    let audio_extensions = vec!["wav", "mkv", "ogg"];
    let video_extensions = vec!["osb", "mp4", "mkv", "flv", "wmv", "mov", "m4v"];
    let image_extensions = vec!["jpg", "png", "jpeg"];

    // concats all the vectors
    let wanted_extensions = [audio_extensions, video_extensions, image_extensions].concat();
    let mut indexed_files = Vec::new();

    // filters out files with the wanted extensions to seperate vector
    let now = Instant::now();
    for entry in WalkDir::new(path.trim()).into_iter().filter_map(|e| e.ok()) {
        let extension = entry.path().extension();
        if extension.map_or(false, |ext| {
            wanted_extensions
                .iter()
                .any(|&x| x == ext.to_ascii_lowercase())
        }) {
            indexed_files.push(entry.path().to_owned())
        }
    }
    let elapsed_time = now.elapsed();
    let amount_of_files = indexed_files.len();

    // asks the user for confirmation they want to delete the files
    println!(
        "\nFound directory {} with {} matching files in {} seconds, do you want to continue?\ny/n",
        path.trim(),
        amount_of_files,
        elapsed_time.as_secs()
    );
    loop {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Invalid response");

        // panics if user indicates they don't want to delete the files
        match response.as_str().trim() {
            "y" => break,
            "n" => panic!("Cancelled deletion"),
            _ => continue,
        }
    }
    (indexed_files, amount_of_files)
}
