use walkdir::WalkDir;

pub fn test_dir() {
    for file in WalkDir::new("/Users/jgkim/Desktop/Dev/project/src")
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        println!("{}", file.path().display());
    }
}
