use std::env::{args};
use std::path::Path;
use std::io;
use std::fs::{read_dir, DirEntry};

fn visible(entry: &DirEntry) -> bool {
    match entry.file_name().to_str().and_then(|file_name| file_name.chars().next()) {
        Some('.') | None => false,
        _ => true
    }
}

fn print_name(path: &Path, depth: &mut Vec<bool>) -> io::Result<()> {
    let file_name = match path.file_name().and_then(|x| x.to_str()) {
        Some(name) => Ok(name.to_string()),
        None => Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
    }?;
    println!("{}{}", get_prefix(&depth), file_name);
    Ok(())
}

fn visit_paths(path: &Path, depth: &mut Vec<bool>) -> io::Result<()> {
    print_name(path, depth);

    if path.is_dir() {
        let mut iter = read_dir(path)?.peekable();

        while let Some(entry) = iter.next() {
            let entry = entry?;
            if visible(&entry) {
                depth.push(!iter.peek().is_some());
                visit_paths(&entry.path(), depth)?;
                depth.pop();
            }
        }
    }
    
    Ok(())
}

fn get_prefix(n: &[bool]) -> String {
    match n.iter().last() {
        None => String::from(""),
        Some(rightmost) => {
            let init = n.iter().take(n.len() - 1).map(
                |b| if *b { "    " } else { "│   " }
            ).collect::<String>();
            let last = if *rightmost { "└── " } else { "├── " };
            format!("{}{}", init, last)
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let pathname = if args.len() < 2 { "." } else { &args[1] };

    let path = Path::new(pathname);

    visit_paths(path, &mut Vec::new())
}
