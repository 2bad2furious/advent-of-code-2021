use std::fs::read_to_string;

pub fn read_asset(path: String) -> String {
    let filename = format!("../assets/{}", path);
    println!("Reading <{}>", filename);
    return read_to_string(filename).unwrap()
}
