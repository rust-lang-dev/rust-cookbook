extern crate ansi_term;
extern crate clap;
mod data_structure;
use clap::{App, Arg};

use data_structure::linked_list;
use data_structure::queue;
use data_structure::stack;
use data_structure::tree;

fn main() {
    let help_info = "Index:\n1.stack\n2.queue\n3.tree\n4.linked_list";

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
        _ => println!("Unknow index"),
    }
}
