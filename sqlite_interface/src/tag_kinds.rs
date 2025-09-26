use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::tags::TagKind;

fn get_contact_kind_from_row(row: &Row) -> Result<TagKind, RusqliteError> {
    Ok(TagKind {
        id: row.get(0)?,
        kind: row.get(1)?,
        deleted_at: row.get(2)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS tag_kinds (
            id INTEGER PRIMARY KEY,
            kind TEXT NOT NULL UNIQUE,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("tag_kinds table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(conn: &mut Connection, id: u64, kind: &str) -> Result<Option<TagKind>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO tag_kinds
            (id, kind)
        VALUES
            (?1, ?2)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut contact_kind_iter = match stmt.query_map((id, kind), get_contact_kind_from_row) {
        Ok(kind_iter) => kind_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_kind_maybe) = contact_kind_iter.next() {
        if let Ok(contact_kind) = contact_kind_maybe {
            return Ok(Some(contact_kind));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<TagKind>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            tag_kinds
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut contact_kind_iter = match stmt.query_map([id], get_contact_kind_from_row) {
        Ok(contact_kind) => contact_kind,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_kind_maybe) = contact_kind_iter.next() {
        if let Ok(contact_kind) = contact_kind_maybe {
            return Ok(Some(contact_kind));
        }
    }

    Ok(None)
}

// read all dbs at scale
pub fn read_by_kind(conn: &mut Connection, kind: &str) -> Result<Option<TagKind>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            tag_kinds
        WHERE
            deleted_at IS NULL
            AND
            kind = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut contact_kind_iter = match stmt.query_map([kind], get_contact_kind_from_row) {
        Ok(kind_iter) => kind_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_kind_maybe) = contact_kind_iter.next() {
        if let Ok(contact_kind) = contact_kind_maybe {
            return Ok(Some(contact_kind));
        }
    }

    Ok(None)
}
