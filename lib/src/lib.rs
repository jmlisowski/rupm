extern crate reqwest;
use dirs;
use colored::Colorize;
use tar::Archive;
use std::path::PathBuf;
use std::io::{self, Read};
use std::fs::{
    File,
    remove_file,
};

fn rupmdir() -> PathBuf {
    let homedir = dirs::home_dir().unwrap();
    let dirpath: PathBuf = (homedir.to_string_lossy().to_string() + "/.rupm").into();
    dirpath
}
fn download(link: &str, filename: &str) {
    let mut resp = reqwest::blocking::get(link).expect("request failed");
    let mut out = File::create(filename).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}
pub fn update() {
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/packages.txt").into();
    let filepath = filepath.to_str().unwrap();
    File::create(&filepath).expect("failed to create file");
    println!("updating package list");
    download("https://raw.githubusercontent.com/jmlisowski/rupm-packages/main/packages.txt", filepath);
}
fn extract(filename: &str) -> io::Result<()> {
    let path = filename;
    let tar = File::open(path)?;
    let mut archive = Archive::new(tar);
    archive.unpack(rupmdir())
}
pub fn install(pkg: &str) {
    let link = format!("https://raw.githubusercontent.com/jmlisowski/rupm-packages/main/{}.tar", pkg);
    let link = link.as_str();
    let filename = format!("{}.tar", pkg);
    let filename = filename.as_str();
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/packages.txt").into();
    let filepath = filepath.to_str().unwrap();
    update();
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file");
    if contents.contains(&pkg) {
        println!("downloading {}.tar", &pkg);
        download(link, &filename);
        println!("extracting {}.tar", &pkg);
        extract(filename).expect("failed to extract tarball");
        println!("cleaning up");
        remove_file(&filename).expect("failed to remove file");
    } else {
        println!("{}",format!("{} is not a package!",&pkg).red().bold());
    }
}
pub fn remove(pkg: &str) {
    let binpath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/bin/").into();
    let pkgpath: PathBuf = (binpath.to_string_lossy().to_string() + pkg).into();
    let pkgpath = pkgpath.to_str().unwrap();
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/packages.txt").into();
    let filepath = filepath.to_str().unwrap();
    update();
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    //if contents.contains(&pkg)
    println!("{}",pkgpath);
}
pub fn help() {
    println!("Commands for rupm:
    help | displays help
    install [package name] | installs a package
    remove [package name] | removes a package");
}