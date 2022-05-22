use std::env;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry};

enum Tree<T> {
    Leaf(T),
    Node(T, Vec<Tree<T>>)
}

fn visible(entry: DirEntry) -> bool {
    match entry.file_name().to_str() {
        Some(file_name) => match file_name.chars().next() {
            Some('.') => false,
            None => false,
            _ => true
        },
        None => false
    }
}

fn visit_dirs(dir: &Path, depth: i32) -> io::Result<Tree<String>> {
    let file_name_x = match dir.file_name().and_then(|x| x.to_str()) {
        Some(file_name) => file_name.to_string(),
        None => ".".to_string(),
    };

    if dir.is_dir() {
        let mut kids = Vec::new();

        let it = fs::read_dir(dir)?;
        for entry in it {
            let entry = entry?;
            let path = entry.path();
            if visible(entry) {
                kids.push(visit_dirs(&path, depth + 1)?);
            }
        }

        Ok(Tree::Node(file_name_x, kids))
    } else {
        Ok(Tree::Leaf(file_name_x))
    }
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("usage: free src/")
    } else {
        let dirname = &args[1];

        let path = Path::new(dirname);

        match visit_dirs(path, 0) {
            Ok(tree) => Ok(view(tree, &mut Vec::new())),
            Err(_) => Err("file system error")
        }
    }
}

fn view(tree: Tree<String>, depth: &mut Vec<bool>) -> () {
    match tree {
        Tree::Leaf(name) => println!("{}{}", get_prefix(&depth), name),
        Tree::Node(name, kids) => {
            println!("{}{}", get_prefix(&depth), name);
            depth.push(true);
            for kid in kids {
                view(kid, depth);
            }
            depth.pop();
        }
    }
}

fn get_prefix(n: &[bool]) -> String {
    // (0..n.len()).map(|_| "    ").collect::<String>()
    match n {
        [] => "".to_string(),
        _ => {
            let n_rev = n.iter().rev();
            n_rev.map(|b| if *b { "│   " } else { "    " } ).collect::<String>()
            // String.concat (List.map (\b -> if b then "    " else "│   ") (List.reverse xs)) ++ (if x then "└── " else "├── ")
        }
    }
}
