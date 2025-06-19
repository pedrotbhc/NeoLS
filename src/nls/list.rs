use std::fs;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

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
                let path = entry.path();
                let perms = permissoes(path.to_str().unwrap())?;
                match file_type {
                    ft if ft.is_dir() => {
                        println!("{} {} {}", perms.yellow(), formatar_tamanho(size), entry.file_name().to_string_lossy().blue());
                    },
                    ft if ft.is_file() => {
                        println!("{} {} {}", perms.yellow(), formatar_tamanho(size), entry.file_name().to_string_lossy().red());
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

fn permissoes(path: &str) -> std::io::Result<String> {
    let metadata = fs::symlink_metadata(path)?;
    let mode = metadata.permissions().mode();

    let file_type = match mode & 0o170000 {
        0o040000 => 'd',
        0o100000 => '-',
        0o120000 => 'l',
        _ => '?'
    };
    let mut perms = String::with_capacity(10);
    perms.push(file_type);
    for i in [6, 3, 0] {
        perms.push(if (mode >> i) & 0b100 != 0 { 'r' } else { '-' });
        perms.push(if (mode >> i) & 0b010 != 0 { 'w' } else { '-' });
        perms.push(if (mode >> i) & 0b001 != 0 { 'x' } else { '-' });
    }
    Ok(perms)
}
