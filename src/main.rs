use prelude::{Result, W};
use std::fs;

mod errors;
mod prelude;
mod utils;

fn main() {
    utils::lib1();
    println!("{:?}", get_files_by_dir("./".to_string()));
}

/// read file list in some dir
fn get_files_by_dir(dir: String) -> Result<Vec<String>> {
    let mut list = vec![];
    for entry in fs::read_dir(dir)?.filter_map(|item| item.ok()) {
        let entry: String = W(&entry).try_into()?;
        list.push(entry);
    }
    return Ok(list);
}
