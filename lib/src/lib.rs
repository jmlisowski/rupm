extern crate reqwest;
use dirs;
use tar::Archive;
use std::path::PathBuf;
use std::io;
use std::fs::{
    File,
    create_dir_all, remove_file,
};

fn rupmdir() -> PathBuf {
    let homedir = dirs::home_dir().unwrap();
    let dirpath: std::path::PathBuf = (homedir.to_string_lossy().to_string() + "/.rupm").into();
    dirpath
}

fn download(link: &str, filename: &str) {
    let resp = reqwest::blocking::get(link).expect("request failed");
    let body = resp.text().expect("body invalid");
    let mut out = File::create(filename).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}
fn extract(filename: &str) -> io::Result<()> {
    let path = filename;
    let tar = File::open(path)?;
    let mut archive = Archive::new(tar);
    archive.unpack(rupmdir())
}
pub fn check() -> bool {
    let dirpath = rupmdir();
    dirpath.exists()
}
pub fn test() {
    let resp = reqwest::blocking::get("https://jmlisowski.github.io/rupm-packages/hello.tar").expect("request failed");
    let body = resp.text().expect("body invalid");
    let mut out = File::create("hello.tar").expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}
pub fn install(pkg: &str) {
    let link = format!("https://jmlisowski.github.io/rupm-packages/{}.tar", pkg);
    let filename = format!("{}.tar", pkg);
    let link = link.as_str();
    let filename = filename.as_str();
    download(&link, &filename);
    extract(&filename).expect("failed to extract tarball");
    remove_file(&filename).expect("failed to remove file");
}

pub fn init() {
    let dirpath = rupmdir();
    let binpath: PathBuf = (dirpath.to_string_lossy().to_string() + "/bin").into();
    println!("creating rupm directory at: {:?}", dirpath);

    set_env::append("PATH", &binpath.to_str().unwrap()).expect("Couldn't find PATH");
    create_dir_all(&binpath).expect("failed to make directory");
}

pub fn help() {
    println!("Commands for rupm: 
    init | sets up rupm for use
    help | displays help
    install [package name] | installs a package
    remove [package name] | removes a package");
}