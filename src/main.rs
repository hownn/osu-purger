mod get_path;
mod index_files;
mod delete_files;

fn main() {
    // gets path for osu! songs directory
    let path = get_path::get_path();

    // files at the given path are indexed into a vector
    let indexed_files = index_files::index_files(path);
    
    // deletes the filtered files
    delete_files::delete_files(indexed_files);
}
