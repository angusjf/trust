use std::env;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry};

fn visit_dirs(dir: &Path) -> io::Result<()> {
    println!("{}", dir.display());
    if dir.is_dir() {
        let mut it = fs::read_dir(dir)?;
        for entry in it {
            // if (it.peek().is_none()) {
            //     print!("!\n");
            // }
            let entry = entry?;
            let path = entry.path();
            visit_dirs(&path)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("usage: free src/")
    } else {
        let dirname = &args[1];

        let path = Path::new(dirname);

        visit_dirs(path);

        Ok(())
    }
}
