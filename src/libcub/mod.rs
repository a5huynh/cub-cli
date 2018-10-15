extern crate chrono;
extern crate rusqlite;
use rusqlite::{ Connection };

pub mod constants;
pub mod note;
use self::note::{ Note, NoteStatus, Tag };

pub enum Limit {
    INFINITE,
    FINITE(i32)
}

const BASE_NOTE_QUERY: &str = "SELECT
        Z_PK,
        ZTITLE,
        ZSUBTITLE,
        ZTEXT,
        ZLASTEDITINGDEVICE,
        datetime(ZCREATIONDATE, 'unixepoch', '+31 years'),
        datetime(ZMODIFICATIONDATE, 'unixepoch', '+31 years'),
        ZARCHIVED,
        ZTRASHED
    FROM ZSFNOTE";

const BASE_TAG_QUERY: &str = "SELECT Z_PK, ZTITLE FROM ZSFNOTETAG ORDER BY ZTITLE";

/// Detect and connect to the Bear application sqlite database.
pub fn connect_to_db(datafile: &str) -> Connection {
    Connection::open(datafile).unwrap()
}

fn apply_filters(query: &str, filters: &[NoteStatus]) -> String {
    let mut filter_sql = Vec::new();
    for filter in filters {
        match filter {
            NoteStatus::ARCHIVED => filter_sql.push("ZARCHIVED = 1"),
            NoteStatus::TRASHED => filter_sql.push("ZTRASHED = 1"),
            NoteStatus::NORMAL => filter_sql.push("(ZARCHIVED = 0 AND ZTRASHED = 0)"),
        }
    }

    if !filter_sql.is_empty() {
        return format!("{} WHERE {}", query, filter_sql.join(" OR "));
    }

    String::from(query)
}


/// Find a single note by ID
pub fn find_note_by_id(conn: &Connection, note_id: i32) -> Result<Note, &'static str> {
    let mut stmt = conn.prepare(format!("{} WHERE Z_PK =?", BASE_NOTE_QUERY).as_str()).unwrap();
    let note = stmt.query_row(&[&note_id], |row| {
        Note::from_sql(row)
    }).unwrap();

    Ok(note)
}


/// List all notes
pub fn list_notes(conn: &Connection, filters: &[NoteStatus], limit: &Limit) -> Result<Vec<Note>, &'static str> {
    let applied = apply_filters(&BASE_NOTE_QUERY, filters);

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
            let note_iter = stmt.query_map(&[val], |row| Note::from_sql(row))
                .unwrap();
            for note in note_iter {
                notes.push(note.unwrap());
            }
        }
    }

    Ok(notes)
}

/// List all tags
pub fn list_tags(conn: &Connection) -> Result<Vec<Tag>, &'static str> {
    let mut stmt = conn.prepare(BASE_TAG_QUERY).unwrap();
    let mut tags = Vec::new();

    let tag_iter = stmt.query_map(&[], |row| Tag::from_sql(row)).unwrap();
    for tag in tag_iter {
        tags.push(tag.unwrap());
    }

    Ok(tags)
}