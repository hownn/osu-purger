mod get_path;
mod index_files;
mod filter_files;
mod delete_files;

fn main() {
    // gets path for osu! songs directory
    let path = get_path::get_path();

    // files at the given path are indexed into a vector
    let indexed_files = index_files::index_files(path);

    // forwards the vector of files to a filter that sorts them based on extension
    let filtered_files = filter_files::filter_files(indexed_files);
    
    // deletes the filtered files
    delete_files::delete_files(filtered_files);
}
