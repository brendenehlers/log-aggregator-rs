use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let dir = Path::new("files");
    files_in_dir(dir)
}

fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn files_in_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files_in_dir(&path)?;       
            } else {
                println!("{}", path.display());
                let msg = read_file(&path)?;
                println!("{}", msg);
            }
        }
    }
    Ok(())
}
