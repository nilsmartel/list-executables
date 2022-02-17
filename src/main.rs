use std::fs::*;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .map(|arg| match arg.as_str() {
            "--path" | "-p" => true,
            "--help" => print_help(0),
            _ => print_help(1),
        })
        .unwrap_or(false);

    let path = std::env::var("PATH").unwrap();

    let readdir = |dir| std::fs::read_dir(dir).unwrap();

    path.split(':')
        .filter(|dir| std::path::Path::new(dir).exists())
        .flat_map(readdir)
        .map(Result::unwrap)
        .filter(|entry| {
            entry
                .metadata()
                // check if entry is a filea and if it's executable
                .map(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
        .for_each(|entry| print_entry(entry, arg));
}

fn print_entry(bin: DirEntry, print_path: bool) {
    if print_path {
        println!("{}", bin.path().to_str().unwrap());
        return;
    }

    println!("{}", bin.file_name().into_string().unwrap());
}

fn print_help(code: i32) -> ! {
    println!(
        "executables [options]
        list all executables that are found in the current PATH

        options:
            -p  --path      print full path to executable
        "
    );

    std::process::exit(code);
}
