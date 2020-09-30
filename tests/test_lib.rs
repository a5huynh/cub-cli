extern crate libcub;
extern crate rusqlite;

use libcub::{list_notes, Limit, SortOrder};
use rusqlite::{params, Connection};

/// Bootstraps a test db with a table with a similar schema to the Bear notes db
/// and some notes.
fn bootstrap(conn: &Connection) {
    conn.execute(
        "CREATE TABLE ZSFNOTE (
            Z_PK                INTEGER PRIMARY KEY,
            ZARCHIVED           INTEGER,
            ZTITLE              VARCHAR,
            ZSUBTITLE           VARCHAR,
            ZTEXT               VARCHAR,
            ZLASTEDITINGDEVICE  VARCHAR,
            ZCREATIONDATE       TIMESTAMP,
            ZMODIFICATIONDATE   TIMESTAMP,
            ZTRASHED            INTEGER)",
        params![],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO ZSFNOTE (
            Z_PK, ZARCHIVED, ZTITLE, ZSUBTITLE, ZTEXT, ZLASTEDITINGDEVICE,
            ZCREATIONDATE, ZMODIFICATIONDATE, ZTRASHED
        ) VALUES (
            1, 0, 'title', 'subtitle', 'text body', 'device', 0, 0, 0
        )",
        params![],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO ZSFNOTE (
            Z_PK, ZARCHIVED, ZTITLE, ZSUBTITLE, ZTEXT, ZLASTEDITINGDEVICE,
            ZCREATIONDATE, ZMODIFICATIONDATE, ZTRASHED
        ) VALUES (
            2, 0, 'title', NULL, NULL, 'device', 0, 0, 0
        )",
        params![],
    )
    .unwrap();
}

#[test]
fn test_list_notes() {
    let conn = Connection::open_in_memory().unwrap();
    bootstrap(&conn);

    let notes = list_notes(&conn, &[], &SortOrder::Title, &[], &Limit::INFINITE).unwrap();
    assert_eq!(notes.len(), 2);
}
