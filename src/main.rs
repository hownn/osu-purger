mod delete_files;
mod get_path;
mod index_files;

fn main() {
    // gets path for files, indexes based on extension and deletes matching files
    delete_files::delete_files(index_files::index_files(get_path::get_path()));
}
