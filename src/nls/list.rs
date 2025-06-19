use std::fs;
use std::io;
use colored::*;

pub struct List;

impl List {

}

pub fn list(show_all: bool, show_metadata: bool, verbose_mode: bool) -> io::Result<()> {
    let list = fs::read_dir(".").unwrap();
    for entry in list {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        match (show_all, show_metadata, verbose_mode) {
            (true, true, true) => {
                let metadata = fs::metadata(entry.path())?;
                let size = metadata.len();
                let type_file = entry.file_type();
                match file_type {
                    ft if ft.is_dir() => {
                        println!("{} {}", formatar_tamanho(size), entry.file_name().to_string_lossy().blue());
                    },
                    ft if ft.is_file() => {
                        println!("{} {}", formatar_tamanho(size), entry.file_name().to_string_lossy().red());
                    },
                    _ => todo!(),
                }
            },
            (true, true, false) => {

            },
            (true, false, false) => {

            },
            (false, false, false) => {
    
            },
            _ => todo!(),
        }
    }
    Ok(())
}

fn formatar_tamanho(bytes: u64) -> String {
    let mut size = bytes as f64;
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut i = 0;
    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }
    format!("{:.2   } {}", size, units[i])
}
