extern crate chrono;
use chrono::prelude::*;
use rusqlite::{Result, Row};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum NoteStatus {
    ARCHIVED,
    TRASHED,
    NORMAL,
}

impl fmt::Display for NoteStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NoteStatus::ARCHIVED => write!(f, "A"),
            NoteStatus::TRASHED => write!(f, "T"),
            NoteStatus::NORMAL => write!(f, "."),
        }
    }
}

#[derive(Debug)]
pub struct Note {
    pub pk: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub text: Option<String>,
    pub last_editing_device: String,
    pub creation_date: NaiveDateTime,
    pub modification_date: NaiveDateTime,
    pub status: NoteStatus,
}

impl Note {
    pub fn from_sql(row: &Row) -> Result<Note> {
        let mut status = NoteStatus::NORMAL;
        if row.get::<usize, i32>(7)? == 1 {
            // Is it archived?
            status = NoteStatus::ARCHIVED;
        } else if row.get::<usize, i32>(8)? == 1 {
            // Is it trashed?
            status = NoteStatus::TRASHED;
        }

        Ok(Note {
            pk: row.get(0)?,
            title: row.get(1)?,
            subtitle: row.get(2)?,
            text: row.get(3)?,
            last_editing_device: row.get(4)?,
            creation_date: NaiveDateTime::parse_from_str(
                row.get::<usize, String>(5)?.as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            modification_date: NaiveDateTime::parse_from_str(
                row.get::<usize, String>(6)?.as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            status,
        })
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:-4} {} {} {}",
            self.pk, self.status, self.modification_date, self.title
        )
    }
}

pub struct Tag {
    pub pk: u32,
    pub title: String,
}

impl Tag {
    pub fn from_sql(row: &Row) -> Result<Tag> {
        Ok(Tag {
            pk: row.get(0)?,
            title: row.get(1)?,
        })
    }
}
