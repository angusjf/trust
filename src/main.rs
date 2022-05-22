use std::env::{args};
use std::path::Path;
use std::io;
use std::fs::{read_dir, DirEntry};

fn visible(entry: &DirEntry) -> bool {
    match entry.file_name().to_str().and_then(|file_name| file_name.chars().next()) {
        None => false,
        Some('.') => false,
        _ => true
    }
}

fn visit_dirs(dir: &Path, depth: &mut Vec<bool>) -> io::Result<()> {
    let file_name_x = match dir.file_name().and_then(|x| x.to_str()) {
        Some(file_name) => Ok(file_name.to_string()),
        None => Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
    }?;

    println!("{}{}", get_prefix(&depth), file_name_x);

    if dir.is_dir() {

        let mut iter = read_dir(dir)?.peekable();

        while let Some(entry) = iter.next() {
            let entry = entry?;
            if visible(&entry) {
                depth.push(!iter.peek().is_some());
                visit_dirs(&entry.path(), depth)?;
                depth.pop();
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let dirname = if args.len() < 2 { "." } else { &args[1] };

    let path = Path::new(dirname);

    visit_dirs(path, &mut Vec::new())
}

fn get_prefix(n: &[bool]) -> String {
    match n.iter().last() {
        None => "".to_string(),
        Some(rightmost) => {
            let init = n.iter().take(n.len() - 1).map(|b| if *b { "    " } else { "│   " } ).collect::<String>();
            let last = if *rightmost { "└── " } else { "├── " };
            format!("{}{}", init, last)
        }
    }
}

