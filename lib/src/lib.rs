use reqwest;
use dirs;
use colored::Colorize;
use tar::Archive;
use serde::{Serialize, Deserialize};
use ron::de::from_reader;
use std::{
    path::PathBuf,
    io::{
        self,
        BufReader,
    },
    fs::{
        File,
        remove_file,
    },
};

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    package: String,
    version: String,
}

fn get_package(name: &str) -> Option<Package> {
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/packages.ron").into();
    let filepath = filepath.to_str().unwrap();
    let file = File::open(filepath).expect("unable to read file");
    let reader = BufReader::new(file);
    let packages: Vec<Package> = from_reader(reader).expect("cannot deserialize");
    for package in packages {
        if package.package == name {
            return Some(package);
        }
    }

    None
}

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
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/packages.ron").into();
    let filepath = filepath.to_str().unwrap();
    File::create(&filepath).expect("failed to create file");
    println!("updating package list");
    download("https://raw.githubusercontent.com/jmlisowski/rupm-packages/main/packages.ron", filepath);
}
fn extract(filename: &str) -> io::Result<()> {
    let path = filename;
    let tar = File::open(path)?;
    let mut archive = Archive::new(tar);
    archive.unpack(rupmdir())
}
pub fn install(pkg: &str) {
    update();
    let package = get_package(pkg);
    match package {
        Some(pkg) => {
            let link = format!("https://raw.githubusercontent.com/jmlisowski/rupm-packages/main/{}.tar", pkg.package);
            let link = link.as_str();
            let filename = format!("{}.tar", pkg.package);
            let filename = filename.as_str();
            println!("downloading {}.tar", pkg.package);
            download(link, filename);
            println!("extracting {}.tar", pkg.package);
            extract(filename).expect("failed to extract tarball");
            println!("cleaning up");
            remove_file(filename).expect("failed to remove file");
        }
        None => {
            println!("{}",format!("{} is not a package!",pkg).red().bold());
        }
    }
}
pub fn remove(pkg: &str) {
    update();
    let package = get_package(pkg);
    match package {
        Some(pkg) => {
            let binpath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/bin/").into();
            let pkgpath: PathBuf = (binpath.to_string_lossy().to_string() + pkg.package.as_str()).into();
            let pkgpath = pkgpath.to_str().unwrap();
            println!("removing {}", pkg.package);
            remove_file(&pkgpath).expect("cannot remove file");
        }
        None => {
            println!("{}",format!("{} is not a package!",pkg).red().bold());
        }
    }
}
pub fn help() {
    println!("Commands for rupm:
    help | displays help
    update | updates package list
    install [package name] | installs a package
    remove [package name] | removes a package");
}