use colored::Colorize;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        if &args[1] != "init" && !lib::check() {
            println!("{}", "rupm not set up yet! run comand 'rupm init' to use rupm.".red().bold());
            exit(1);
        }
        if &args[1] == "help" {
            lib::help();
        } else if &args[1] == "install" {
            lib::install(&args[2]);
        } else if &args[1] == "init" {
            lib::init();
        } else if &args[1] == "update" {
            lib::update();
        } else {
            println!("{} '{}'", "couldn't recognize command".red().bold(), &args[1].red().bold());
            lib::help();
        }
    } else {
        if !lib::check() {
            println!("{}", "rupm not set up yet! run comand 'rupm init' to use rupm.".red().bold());
            exit(1);
        }
        lib::help();
    }
}
