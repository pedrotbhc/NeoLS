use std::fs;



fn main() {
    let list = fs::read_dir(".").unwrap();

    for entry in list {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }

}
