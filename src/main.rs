use colored::Colorize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        if &args[1] == "help" {
            lib::help();
        } else if &args[1] == "test" {
            lib::add_installed_package("test".to_string(), "1.0.2".to_string());
        } else if &args[1] == "install" {
            for (i, arg) in args.iter().enumerate() {
                if i > 1 {
                    lib::install(arg);
                }
            }
        } else if &args[1] == "remove" {
            for (i, arg) in args.iter().enumerate() {
                if i > 1 {
                    lib::remove(arg);
                }
            }
        } else if &args[1] == "update" {
            lib::update();
        } else if &args[1] == "upgrade" {
            lib::upgrade();
        } else {
            println!("{} '{}'", "couldn't recognize command".red().bold(), &args[1].red().bold());
            lib::help();
        }
    } else {
        lib::help();
    }
}
