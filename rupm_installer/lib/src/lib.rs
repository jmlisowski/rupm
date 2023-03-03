use std::fs::create_dir_all;
use reqwest;
use dirs;
use colored::Colorize;
use tar::Archive;
use serde::{Serialize, Deserialize};
use ron::{
    de::from_reader,
    ser::{
        to_string_pretty,
        PrettyConfig,
    }
};
use std::{
    path::PathBuf,
    io::{
        self,
        Seek,
        SeekFrom,
        BufReader,
        Write,
        Read,
    },
    fs::{
        OpenOptions,
        File,
        remove_file,
    },
};

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    package: String,
    version: String,
}
struct Place {
    exists: bool,
    index: usize,
}
fn is_package_installed(package: &String) -> Place {
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/installedpackages.ron").into();
    let mut file = OpenOptions::new()
        .read(true)        
        .open(&filepath)
        .unwrap();
    let packages: Vec<Package> = from_reader(BufReader::new(&mut file)).unwrap();
    for (i, pkg) in packages.iter().enumerate() {
        if &pkg.package == package {
            return Place {
                exists: true,
                index: i,
            };
        }
    }
    Place {
        exists: false,
        index: 0,
    }
}
pub fn add_installed_package(package: String, version: String,) {
    let data = Package {
        package,
        version,
    };
    let filepath: PathBuf = (rupmdir().to_string_lossy().to_string() + "/installedpackages.ron").into();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&filepath)
        .unwrap();

    let mut installedpackages = File::open(&filepath).expect("failed to open file");
    let mut contents = String::new();
    installedpackages.read_to_string(&mut contents).expect("failed to write to string");
    let installedpkglist = match contents.chars().count() {
        0..=1 => {
            let mut packages = Vec::new();
            packages.push(data);
            packages
        },
        _ => {
            let mut packages: Vec<Package> = from_reader(BufReader::new(&mut file)).unwrap();
            if is_package_installed(&data.package).exists {
                println!("{:?}, {}", packages, is_package_installed(&data.package).index);
                packages.remove(is_package_installed(&data.package).index);
            };
            packages.push(data);
            packages
        },
    };

    let s = to_string_pretty(&installedpkglist, PrettyConfig::default()).expect("cannot deserialize");
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(s.as_bytes()).expect("failed to write to file");
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
fn update() {
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
fn install(pkg: &str) {
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
            add_installed_package(pkg.package, pkg.version);
        }
        None => {
            println!("{}",format!("{} is not a package!",pkg).red().bold());
        }
    }
}
pub fn init() {
    let dirpath = rupmdir();
    let binpath: PathBuf = (dirpath.to_string_lossy().to_string() + "/bin").into();
    let installedpackages: PathBuf = (dirpath.to_string_lossy().to_string() + "/installedpackages.ron").into();
    println!("creating rupm directory at: {:?}", dirpath);
    println!("adding {:?} to PATH", binpath);

    set_env::append("PATH", &binpath.to_str().unwrap()).expect("Couldn't find PATH");
    create_dir_all(&binpath).expect("failed to make directory");
    File::create(installedpackages).expect("failed to create fie");

    update();
    install("rupm");
    println!("{}", "rupm is sccessfuly installed! You can delete this file and log out and log back in.".green().bold());
}