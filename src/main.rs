use colored::Colorize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !lib::check() && &args[1] != "init" {
        println!("{}", "rupm not set up yet! run comand 'rupm init' to use rupm.".red().bold());
    }
    if args.len() > 1 {
        if &args[1] == "help" {
            lib::help();
        } else if &args[1] == "install" {
            lib::install(&args[2]);
        } else if &args[1] == "test" {
            lib::test();
        } else if &args[1] == "init" {
            lib::init();
        } else {
            println!("{} '{}'", "couldn't recognize command".red().bold(), &args[1].red().bold());
            lib::help();
        }
    } else {
        lib::help();
    }
}
