use std::env;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry};

fn visible(entry: &DirEntry) -> bool {
    match entry.file_name().to_str() {
        Some(file_name) => match file_name.chars().next() {
            Some('.') => false,
            None => false,
            _ => true
        },
        None => false
    }
}

fn visit_dirs(dir: &Path, depth: &mut Vec<bool>) -> io::Result<()> {
    let file_name_x = match dir.file_name().and_then(|x| x.to_str()) {
        Some(file_name) => file_name.to_string(),
        None => ".".to_string(),
    };

    if dir.is_dir() {
        println!("{}{}", get_prefix(&depth), file_name_x);

        let mut iter = fs::read_dir(dir)?.peekable();

        while let Some(entry) = iter.next() {
            let entry = entry?;
            if visible(&entry) {
                depth.push(!iter.peek().is_some());
                visit_dirs(&entry.path(), depth)?;
                depth.pop();
            }
        }

        Ok(())
    } else {
        println!("{}{}", get_prefix(&depth), file_name_x);

        Ok(())
    }
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    let dirname = if args.len() < 2 { "." } else { &args[1] };

    let path = Path::new(dirname);

    match visit_dirs(path, &mut Vec::new()) {
        Ok(()) => Ok(()),
        Err(_x) => Err("file system error")
    }
}

fn get_prefix(n: &[bool]) -> String {
    match n.iter().last() {
        None => "".to_string(),
        Some(b1) => {
            let init = n.iter().take(n.len() - 1).map(|b| if *b { "    " } else { "│   " } ).collect::<String>();
            let last = if *b1 { "└── " } else { "├── " };
            format!("{}{}", init, last)
        }
    }
}

