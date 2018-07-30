use rusqlite::{ Connection };

pub mod constants;
pub mod note;
use self::note::{ Note, NoteStatus };

use args::Limit;

const BASE_QUERY: &'static str = "SELECT
        Z_PK,
        ZTITLE,
        ZSUBTITLE,
        ZTEXT,
        ZLASTEDITINGDEVICE,
        strftime('%s', ZCREATIONDATE),
        strftime('%s', ZMODIFICATIONDATE),
        ZARCHIVED,
        ZTRASHED
    FROM ZSFNOTE";

/// Detect and connect to the Bear application sqlite database.
pub fn connect_to_db(datafile: &str) -> Connection {
    return Connection::open(datafile).unwrap();
}

fn apply_filters(query: &str, filters: &Vec<NoteStatus>) -> String {
    let mut filter_sql = Vec::new();
    for filter in filters {
        match filter {
            NoteStatus::ARCHIVED => filter_sql.push("ZARCHIVED = 1"),
            NoteStatus::TRASHED => filter_sql.push("ZTRASHED = 1"),
            NoteStatus::NORMAL => filter_sql.push("(ZARCHIVED = 0 AND ZTRASHED = 0)"),
        }
    }

    if filter_sql.len() > 0 {
        return format!("{} WHERE {}", query, filter_sql.join(" OR "));
    }

    return String::from(query);
}


/// Find a single note by ID
pub fn find_note_by_id(conn: &Connection, note_id: i32) -> Result<Note, &'static str> {
    let mut stmt = conn.prepare(format!("{} WHERE Z_PK =?", BASE_QUERY).as_str()).unwrap();
    let note = stmt.query_row(&[&note_id], |row| {
        Note::from_sql(row)
    }).unwrap();

    return Ok(note);

}


/// List all notes
pub fn list_notes(conn: &Connection, filters: &Vec<NoteStatus>, limit: Limit) -> Result<Vec<Note>, &'static str> {
    let applied = apply_filters(&BASE_QUERY, filters);

    let mut notes = Vec::new();

    match limit {
        // Show all notes
        Limit::INFINITE => {
            let mut stmt = conn.prepare(&applied.as_str())
                .unwrap();
            let note_iter = stmt.query_map(&[], |row| Note::from_sql(row))
                .unwrap();
            for note in note_iter {
                notes.push(note.unwrap());
            }
        },
        // Apply limit to number of notes returned
        Limit::FINITE(val) => {
            let mut stmt = conn.prepare(format!("{} LIMIT ?", &applied.as_str()).as_str())
                .unwrap();
            let note_iter = stmt.query_map(&[&val], |row| Note::from_sql(row))
                .unwrap();
            for note in note_iter {
                notes.push(note.unwrap());
            }
        }
    }

    return Ok(notes);
}