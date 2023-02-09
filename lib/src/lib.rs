extern crate reqwest;
use dirs;
use tar::Archive;
use std::path::PathBuf;
use std::io;
use std::fs::{
    File,
    create_dir,
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
pub fn check() -> bool {
    let dirpath = rupmdir();
    dirpath.exists()
}
pub fn test() -> io::Result<()> {
    let resp = reqwest::blocking::get("https://jmlisowski.github.io/rupm/hello.tar").expect("request failed");
    let body = resp.text().expect("body invalid");
    let mut out = File::create("hello.tar").expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
    Ok(())
}
pub fn install(pkg: &str) -> Result<(), io::Error>{
    let link = format!("https://jmlisowski.github.io/rupm/{}.tar", pkg);
    let filename = format!("{}.tar", pkg);
    let link = link.as_str();
    let filename = filename.as_str();
    download(&link, &filename);

    let path = filename;
    let tar = File::open(path)?;
    let mut archive = Archive::new(tar);
    archive.unpack(rupmdir())
}
pub fn init() -> io::Result<()> {
    let dirpath = rupmdir();    
    println!("creating rupm directory at: {:?}", dirpath);
    create_dir(dirpath)
}

pub fn help() -> io::Result<()> {
    println!("Commands for rupm: 
    init | sets up rupm for use
    help | displays help
    install [package name] | installs a package
    remove [package name] | removes a package");
    Ok(())
}