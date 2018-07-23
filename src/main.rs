extern crate chrono;
#[macro_use]
extern crate clap;
extern crate rusqlite;
extern crate term;

use clap::App;
use std::io::prelude::*;

mod libcub;
use libcub::{
    connect_to_db,
    list_notes
};


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Attempt to detect and connect to the Bear sqlite database
    let conn = connect_to_db();
    if let Some(matches) = matches.subcommand_matches("ls") {
        let mut t = term::stdout().unwrap();

        let mut limit = 100;
        if matches.is_present("limit") {
            let limit_str = matches.value_of("limit").unwrap();
            limit = limit_str.parse::<i32>().expect("Limit needs to be a number.");
        }

        let notes = list_notes(&conn, limit).unwrap();
        for note in notes {

            t.fg(term::color::GREEN).unwrap();
            writeln!(t, "{}", note.title).unwrap();

            if matches.is_present("text") {
                t.fg(term::color::BRIGHT_WHITE).unwrap();
                writeln!(t, "{}", note.subtitle).unwrap();
            }

            t.reset().unwrap();

        }
    }
}
