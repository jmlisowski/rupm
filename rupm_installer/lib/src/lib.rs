extern crate reqwest;
use dirs;
use colored::Colorize;
use tar::Archive;
use std::path::PathBuf;
use std::io::{self, Read};
use std::fs::{
    File,
    create_dir_all,
};
fn rupmdir() -> PathBuf {
    let homedir = dirs::home_dir().unwrap();
    let dirpath: PathBuf = (homedir.to_string_lossy().to_string() + "/.rupm").into();
    dirpath
}
fn update() {
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/packages.txt").into();
    let filepath = filepath.to_string_lossy().to_string();
    let filepath = filepath.as_str();
    File::create(&filepath).expect("failed to create file");
    println!("updating package list");
    download("https://raw.githubusercontent.com/jmlisowski/rupm-packages/main/packages.txt", filepath);
}
pub fn install() {
    let dirpath = rupmdir();
    let binpath: PathBuf = (dirpath.to_string_lossy().to_string() + "/bin").into();
    println!("creating rupm directory at: {:?}", dirpath);
    println!("adding {:?} to PATH", binpath);

    set_env::append("PATH", &binpath.to_str().unwrap()).expect("Couldn't find PATH");
    create_dir_all(&binpath).expect("failed to make directory");

    update();
}