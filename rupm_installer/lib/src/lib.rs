extern crate reqwest;
use dirs;
use colored::Colorize;
use tar::Archive;
use std::path::PathBuf;
use std::io::{self, Read};
use std::fs::{
    File,
    create_dir_all, remove_file,
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
fn update() {
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/packages.txt").into();
    let filepath = filepath.to_string_lossy().to_string();
    let filepath = filepath.as_str();
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
fn install(pkg: &str) {
    let link = format!("https://raw.githubusercontent.com/jmlisowski/rupm-packages/main/{}.tar", pkg);
    let filename = format!("{}.tar", pkg);
    let link = link.as_str();
    let filename = filename.as_str();
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/packages.txt").into();
    let filepath = filepath.to_string_lossy().to_string();
    let filepath = filepath.as_str();
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
pub fn init() {
    let dirpath = rupmdir();
    let binpath: PathBuf = (dirpath.to_string_lossy().to_string() + "/bin").into();
    println!("creating rupm directory at: {:?}", dirpath);
    println!("adding {:?} to PATH", binpath);

    set_env::append("PATH", &binpath.to_str().unwrap()).expect("Couldn't find PATH");
    create_dir_all(&binpath).expect("failed to make directory");

    update();
    install("rupm");
    println!("{}", "rupm is sccessfuly installed! You can now delete this file.".green().bold());
}