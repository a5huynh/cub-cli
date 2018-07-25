extern crate chrono;
#[macro_use]
extern crate clap;
extern crate rusqlite;
extern crate term;

use clap::App;
use std::io::prelude::*;

mod args;
use args::{
    parse_filters,
    parse_limit,
};

mod libcub;
use libcub::{
    connect_to_db,
    list_notes,
    NoteStatus,
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Attempt to detect and connect to the Bear sqlite database
    let conn = connect_to_db();

    let mut t = term::stdout().unwrap();

    // Parse command line args and determine which subcommand to execute.
    if let Some(matches) = matches.subcommand_matches("ls") {

        let filters = parse_filters(matches);
        let limit = parse_limit(matches);

        for note in list_notes(&conn, &filters, limit).unwrap() {
            // Color the notes depending on the note status
            if matches.is_present("color") {
                match note.status {
                    NoteStatus::NORMAL => t.fg(term::color::WHITE).unwrap(),
                    NoteStatus::TRASHED => t.fg(term::color::RED).unwrap(),
                    NoteStatus::ARCHIVED => t.fg(term::color::GREEN).unwrap(),
                }
            }

            writeln!(t, "{}", note.title).unwrap();
            if matches.is_present("text") {
                writeln!(t, "{}", note.subtitle).unwrap();
            }

            if matches.is_present("color") {
                t.reset().unwrap();
            }
        }
    }
}
