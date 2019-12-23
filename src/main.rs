use std::fs::*;
fn main() {
    let arg = if let Some(arg) = std::env::args().nth(1) {
        &arg == "--path" || &arg == "-p"
    } else {
        false
    };

    let path = std::env::var("PATH").unwrap();

    let readdir = |dir| std::fs::read_dir(dir).unwrap();

    path.split(':')
        .filter(|dir| std::path::Path::new(dir).exists())
        .flat_map(readdir)
        .for_each(|entry| print_entry(entry.unwrap(), arg));
}

fn print_entry(bin: DirEntry, print_path: bool) {
    if print_path {
        println!("{}", bin.path().to_str().unwrap());
    } else {
        println!("{}", bin.file_name().into_string().unwrap());
    }
}
