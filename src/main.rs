use std::env;

use crate::cli::build_cli;

mod cli;
mod data;
mod event;
mod input;
mod key;
mod query;

fn main() {
    // for line in io::stdin().lock().lines() {
    //     println!("{:?}", line);
    // }

    let matches = build_cli().get_matches_from(env::args_os());
    println!("{:?}", matches.value_of("input"));
    if matches.is_present("filetype") {
        let value = matches.value_of("filetype");
        println!("{:?}", value)
    }
}
