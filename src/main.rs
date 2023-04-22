mod lib;

fn main() {
    // gets path for osu! songs directory
    let path = lib::get_path();

    // files at the given path are indexed into a vector
    let indexed_files = lib::index_files(path);

    // forwards the vector of files to a filter that sorts them based on extension
    let filtered_files = lib::filter_files(indexed_files);
    
    // deletes the filtered files
    lib::delete_files(filtered_files);
}
