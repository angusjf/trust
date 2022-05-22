use std::env::args;
use std::fs::{read_dir, DirEntry};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

fn visible(entry: &DirEntry) -> bool {
    match entry
        .file_name()
        .to_str()
        .and_then(|name| name.chars().next())
    {
        Some('.') | None => false,
        _ => true,
    }
}

fn print_name(path: &Path, depth: &mut Vec<bool>) -> Result<()> {
    let file_name = match path.file_name().and_then(|x| x.to_str()) {
        Some(name) => Ok(name.to_string()),
        None => Err(Error::new(ErrorKind::Other, "oh no!")),
    }?;
    println!("{}{}", get_prefix(&depth), file_name);
    Ok(())
}

fn visit_paths(path: &Path, depth: &mut Vec<bool>) -> Result<()> {
    print_name(path, depth);

    if path.is_dir() {
        let mut iter = read_dir(path)?.peekable();

        while let Some(Ok(entry)) = iter.next() {
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
        Some(rightmost) => {
            let init = n
                .iter()
                .take(n.len() - 1)
                .map(|b| if *b { "    " } else { "│   " })
                .collect::<String>();
            let last = if *rightmost {
                "└── "
            } else {
                "├── "
            };
            format!("{}{}", init, last)
        }
        None => String::from(""),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    let path = Path::new(if args.len() < 2 { "." } else { &args[1] });

    if path.exists() {
        visit_paths(path, &mut Vec::new())
    } else {
        Err(Error::new(ErrorKind::Other, "file does not exist"))
    }
}
