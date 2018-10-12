#[macro_use]
extern crate clap;
extern crate dirs;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate term;

use clap::App;
use std::io::prelude::*;

mod args;
use args::{
    parse_filters,
    parse_limit,
};

extern crate libcub;
use libcub::{
    connect_to_db,
    find_note_by_id,
    list_notes,
    list_tags,
};
use libcub::constants::{ find_db };
use libcub::note::NoteStatus;

fn main() {
    env_logger::init();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut t = term::stdout().unwrap();

    // Find the path to the Bear sqlite file.
    // I assume that the sqlite file is in the same place for all installs,
    // but make that option configurable.
    let db_file_path = match find_db() {
        Ok(db_path) => db_path,
        Err(message) => {
            eprint!("{}", message);
            return;
        }
    };

    let db_opt = matches.value_of("db").unwrap_or_else(|| db_file_path.as_str());

    info!("db path set to: {}", db_opt);

    // Attempt to detect and connect to the Bear sqlite database
    let conn = connect_to_db(db_opt);

    // Parse command line args and determine which subcommand to execute.
    if let Some(matches) = matches.subcommand_matches("ls") {

        let filters = parse_filters(matches);
        let limit = parse_limit(matches);

        for note in list_notes(&conn, &filters, &limit).unwrap() {
            // Color the notes depending on the note status
            if matches.is_present("color") {
                match note.status {
                    NoteStatus::NORMAL => t.fg(term::color::WHITE).unwrap(),
                    NoteStatus::TRASHED => t.fg(term::color::RED).unwrap(),
                    NoteStatus::ARCHIVED => t.fg(term::color::GREEN).unwrap(),
                }
            }

            writeln!(t, "{:-4} {}", note.pk, note.title).unwrap();
            if matches.is_present("full") {
                writeln!(t, "{}", note.subtitle).unwrap();
            }

            // Unset any coloring we did
            if matches.is_present("color") {
                t.reset().unwrap();
            }
        }

    } else if let Some(matches) = matches.subcommand_matches("show") {

        let note_id: i32 = match matches.value_of("NOTE").unwrap().parse() {
            Ok(value) => value,
            Err(_) => {
                writeln!(t, "Note id must be a valid integer.").unwrap();
                ::std::process::exit(1);
            }
        };

        let note = find_note_by_id(&conn, note_id).unwrap();
        writeln!(t, "{}", note.text).unwrap();

    } else if matches.subcommand_matches("tags").is_some() {
        for tag in list_tags(&conn).unwrap() {
            writeln!(t, "{}", tag.title).unwrap();
        }
    }
}
