use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::tags::Tag;

fn get_tag_from_row(row: &Row) -> Result<Tag, RusqliteError> {
    Ok(Tag {
        id: row.get(0)?,
        people_id: row.get(1)?,
        tag_kind_id: row.get(2)?,
        bookmark_id: row.get(3)?,
        deleted_at: row.get(4)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            people_id INTEGER NOT NULL,
            tag_kind_id INTEGER NOT NULL,
            bookmark_id INTEGER NOT NULL,
            deleted_at INTEGER,
            UNIQUE (tag_kind_id, bookmark_id)
        )",
        (),
    );

    if let Err(e) = results {
        return Err("tags table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    people_id: u64,
    tag_kind_id: u64,
    bookmark_id: u64,
) -> Result<Option<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO tags
            (id, people_id, tag_kind_id, bookmark_id)
        VALUES
            (?1, ?2, ?3, ?4)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to create a contact".to_string()),
    };

    let mut tag_iter = match stmt.query_map(
        (id, people_id, tag_kind_id, bookmark_id),
        get_tag_from_row,
    ) {
        Ok(tag_iter) => tag_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(tag_maybe) = tag_iter.next() {
        if let Ok(tag) = tag_maybe {
            return Ok(Some(tag));
        }
    }

    Ok(None)
}

pub fn read_by_id(conn: &mut Connection, id: u64) -> Result<Option<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            tags
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read a contact".to_string()),
    };

    let mut tag_iter = match stmt.query_map([id], get_tag_from_row) {
        Ok(tag_iter) => tag_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(tag_maybe) = tag_iter.next() {
        if let Ok(tag) = tag_maybe {
            return Ok(Some(tag));
        }
    }

    Ok(None)
}

pub fn read_by_tag_kind_id(
    conn: &mut Connection,
    tag_kind_id: u64,
) -> Result<Option<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            tags
        WHERE
            deleted_at IS NULL
            AND
            tag_kind_id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read a contact by id".to_string()),
    };

    let mut tag_iter = match stmt.query_map([tag_kind_id], get_tag_from_row) {
        Ok(tag_iter) => tag_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(tag_maybe) = tag_iter.next() {
        if let Ok(tag) = tag_maybe {
            return Ok(Some(tag));
        }
    }

    Ok(None)
}
