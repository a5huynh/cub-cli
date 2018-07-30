extern crate chrono;
#[macro_use]
extern crate clap;
extern crate dirs;
#[macro_use]
extern crate log;
extern crate env_logger;
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
    find_note_by_id,
    list_notes
};
use libcub::constants::{ APP_ROOT, DB_PATH };
use libcub::note::NoteStatus;

fn main() {
    env_logger::init();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Find the path to the Bear sqlite file.
    let default_db_path = dirs::home_dir().unwrap().join(APP_ROOT).join(DB_PATH);
    let db_opt = matches.value_of("db").unwrap_or(default_db_path.to_str().unwrap());
    info!("db path set to: {}", db_opt);

    // Attempt to detect and connect to the Bear sqlite database
    let conn = connect_to_db(db_opt);
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

            writeln!(t, "{:-4} {}", note.pk, note.title).unwrap();
            if matches.is_present("text") {
                writeln!(t, "{}", note.subtitle).unwrap();
            }

            // Unset any coloring we did
            if matches.is_present("color") {
                t.reset().unwrap();
            }
        }

    } else if let Some(matches) = matches.subcommand_matches("show") {

        let note_id = matches.value_of("NOTE").unwrap().parse::<i32>().unwrap();
        let note = find_note_by_id(&conn, note_id).unwrap();
        writeln!(t, "{}", note.text).unwrap();

    }
}
