use std::fs;
use std::io;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;

use colored::*;
use users::get_user_by_uid;
    
pub fn listar(_show_all: bool, _show_long: bool) -> io::Result<()> {
    let list = fs::read_dir(".")?;
    for entry_res in list {
        let entry = entry_res?;
        let metadata = fs::metadata(entry.path())?;
        let byte_size = metadata.len();
        let size = tamanho(byte_size);
        let file_type = entry.file_type()?;
        let path = entry.path();
        let perms = permissoes(path.to_str().unwrap())?;
        match file_type {
            ft if ft.is_dir() => {
                println!("{} {} {} {}",
                    perms.yellow(),
                    dono(&path).unwrap_or("?".to_string()).to_string(),
                    size,
                    entry.file_name().to_string_lossy().blue().bold());
            },
            ft if ft.is_file() => {
		println!("{} {} {} {}",
                    perms.yellow(),
                    dono(&path).unwrap_or("?".to_string()).to_string(),
                    size,
                    entry.file_name().to_string_lossy());
            },
            _ => todo!(),
        }
    }
    Ok(())
}

fn tamanho(bytes: u64) -> String {
    let mut size = bytes as f64;
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut i = 0;
    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }
    format!("{:.2} {}", size, units[i])
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

fn dono(path: &Path) -> Option<String> {
    let metadata = fs::metadata(&path).ok()?;
    let uid = metadata.uid();
    let user = get_user_by_uid(uid)?;

    Some(user.name().to_string_lossy().into_owned())
}
