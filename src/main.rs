fn main() {
    let path = std::env::var("PATH").unwrap();

    let readdir = |dir| std::fs::read_dir(dir).unwrap();

    path.split(':')
        .filter(|dir| std::path::Path::new(dir).exists())
        .flat_map(readdir)
        .for_each(|bin| {
            println!("{}", bin.unwrap().file_name().into_string().unwrap());
        });
}
