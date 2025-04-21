use std::{
    env::{self, args},
    path::{Path, PathBuf},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Expect 1 argument as path");
    }

    let path = &args[1];

    let mut path_buf = PathBuf::new();
    path_buf.push(path);

    if path_buf.is_dir() {
        scan_and_rename_dir(path_buf);
        return;
    }

    rename_file(path_buf);
}

fn rename_file(path: impl AsRef<Path>) {
    let file_name = path
        .as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let new_file_name = file_name.replace('_', " ").replace("dokumen.pub", "");
    let new_path = Path::join(path.as_ref().parent().unwrap(), new_file_name);

    std::fs::rename(path, new_path).unwrap();
}

fn scan_and_rename_dir(dir: impl AsRef<Path>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "pdf" {
                    rename_file(&path);
                }
            }
        }
    }
}
