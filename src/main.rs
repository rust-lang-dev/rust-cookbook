extern crate ansi_term;
extern crate clap;
mod basic;
mod data_structure;
mod file;
use clap::{App, Arg};

use basic::*;
use data_structure::*;
use file::*;

fn main() {
    let help_info = "Index:\n
        1.stack 2.queue 3.tree\n
        4.linked list 5.string 6.format\n
        7.builder 8.env 9. basic file\n";

    let matches = App::new("Hello World")
        .version("0.0.1")
        .author("Ben")
        .about("Rust Cookbook")
        .arg(
            Arg::with_name("index")
                .short("i")
                .long("index")
                .takes_value(true)
                .help(help_info),
        )
        .get_matches();

    let index_str = matches.value_of("index");
    match index_str {
        None => println!("Index is required"),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => run_receipt(n),
            Err(_) => println!("Not a number {}", s),
        },
    }
}

fn run_receipt(index: i32) {
    match index {
        1 => stack::test(),
        2 => queue::test(),
        3 => tree::test(),
        4 => linked_list::test(),
        5 => string::test(),
        6 => format::test(),
        7 => builder::test(),
        8 => env::test(),
        9 => basic_file::test(),
        _ => println!("Unknown index"),
    }
}
