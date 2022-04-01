use std::fs::File;
use std::io::prelude::*;

pub fn save_edge(filename: &str, edge_vec: &[u32]) -> Result<(), ()> {
    let mut file = File::create(filename).expect("Failed to write edge info");
    let mut edge_str = String::new();

    for i in edge_vec.iter() {
        edge_str.push_str(&(i.to_string()));
        edge_str.push_str(" ");
    }
    file.write_all(edge_str.as_bytes()).unwrap();
    Ok(())
}
