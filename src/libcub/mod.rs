extern crate chrono;

use chrono::prelude::*;
use rusqlite::{ Connection };

#[derive(Debug)]
pub enum NoteStatus {
    ARCHIVED,
    TRASHED,
    NORMAL
}

#[derive(Debug)]
pub struct Note {
    pub pk: i32,
    pub title: String,
    pub subtitle: String,
    pub text: String,
    pub last_editing_device: String,
    pub creation_date: NaiveDateTime,
    pub modification_date: NaiveDateTime,
    pub status: NoteStatus
}


/// Detect and connect to the Bear application sqlite database.
pub fn connect_to_db() -> Connection {
    // TODO: Point this at the real bear application sqlite db.
    return Connection::open("data/database.sqlite").unwrap();
}


/// List all notes
pub fn list_notes(conn: &Connection, limit: i32) -> Result<Vec<Note>, &'static str> {

    let mut stmt = conn
        .prepare("SELECT
                Z_PK,
                ZTITLE,
                ZSUBTITLE,
                ZTEXT,
                ZLASTEDITINGDEVICE,
                strftime('%s', ZCREATIONDATE),
                strftime('%s', ZMODIFICATIONDATE),
                ZARCHIVED,
                ZTRASHED
            FROM ZSFNOTE
            LIMIT ?")
        .unwrap();

    let note_iter = stmt.query_map(&[&limit], |row| {
        let mut status = NoteStatus::NORMAL;
        if row.get::<i32, i32>(7) == 1 {
            // Is it archived?
            status = NoteStatus::ARCHIVED;
        } else if row.get::<i32, i32>(8) == 1 {
            // Is it trashed?
            status = NoteStatus::TRASHED;
        }

        Note {
            pk: row.get(0),
            title: row.get(1),
            subtitle: row.get(2),
            text: row.get(3),
            last_editing_device: row.get(4),
            creation_date: NaiveDateTime::from_timestamp(
                row.get_checked(5).unwrap_or(0),
                0
            ),
            modification_date: NaiveDateTime::from_timestamp(
                row.get_checked(6).unwrap_or(0),
                0
            ),
            status: status
        }
    }).unwrap();

    let mut new = Vec::new();
    for note in note_iter {
        new.push(note.unwrap());
    }

    return Ok(new);
}