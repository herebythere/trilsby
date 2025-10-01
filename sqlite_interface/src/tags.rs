use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::Tag;

fn get_entry_from_row(row: &Row) -> Result<Tag, RusqliteError> {
    Ok(Tag {
        id: row.get(0)?,
        tag_kind_id: row.get(1)?,
        bookmark_id: row.get(2)?,
        people_id: row.get(3)?,
        deleted_at: row.get(4)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            tag_kind_id INTEGER NOT NULL,
            bookmark_id INTEGER NOT NULL,
            people_id INTEGER NOT NULL,
            deleted_at INTEGER,
            UNIQUE (tag_kind_id, bookmark_id)
        )",
        (),
    );

    if let Err(_e) = results {
        return Err("failed to create tags table".to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    tag_kind_id: u64,
    bookmark_id: u64,
    people_id: u64,
) -> Result<Option<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO tags
            (id, tag_kind_id, bookmark_id, people_id)
        VALUES
            (?1, ?2, ?3, ?4)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to create tag".to_string()),
    };

    let mut tag_iter = match stmt.query_map(
        (id, tag_kind_id, bookmark_id, people_id),
        get_entry_from_row,
    ) {
        Ok(tag_iter) => tag_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(entry_maybe) = tag_iter.next() {
        if let Ok(tag) = entry_maybe {
            return Ok(Some(tag));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, limit: u64, offset: u64) -> Result<Vec<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            tags
        WHERE
            deleted_at IS NULL
        ORDER BY
            id DESC
        LIMIT
            ?1
        OFFSET
            ?2
        ",
    ) {
        Ok(stmt) => stmt,
        Err(_e) => return Err("failed to read tags".to_string()),
    };

    let mut tag_iter = match stmt.query_map((limit, offset), get_entry_from_row) {
        Ok(tag_iter) => tag_iter,
        Err(e) => return Err(e.to_string()),
    };

    let mut tags: Vec<Tag> = Vec::new();
    while let Some(entry_maybe) = tag_iter.next() {
        if let Ok(tag) = entry_maybe {
            tags.push(tag);
        }
    }

    Ok(tags)
}

pub fn read_by_tag_kind_id(
    conn: &mut Connection,
    tag_kind_id: u64,
    limit: u64,
    offset: u64,
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
        LIMIT
            ?2
        OFFSET
            ?3
        ORDER BY
            id DESC
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read tags by tag_kind_id".to_string()),
    };

    let mut tag_iter = match stmt.query_map((tag_kind_id, limit, offset), get_entry_from_row) {
        Ok(tag_iter) => tag_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(entry_maybe) = tag_iter.next() {
        if let Ok(tag) = entry_maybe {
            return Ok(Some(tag));
        }
    }

    Ok(None)
}

// limit offset ascending descending
pub fn read_by_bookmark_id(
    conn: &mut Connection,
    bookmark_id: u64,
    limit: u64,
    offset: u64,
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
            bookmark_id = ?1
        LIMIT
            ?2
        OFFSET
            ?3
        ORDER BY
            id DESC
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read tags by bookmark_id".to_string()),
    };

    let mut tag_iter = match stmt.query_map((bookmark_id, limit, offset), get_entry_from_row) {
        Ok(tag_iter) => tag_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(entry_maybe) = tag_iter.next() {
        if let Ok(tag) = entry_maybe {
            return Ok(Some(tag));
        }
    }

    Ok(None)
}

// limit offset ascending descending
pub fn read_by_people_id(
    conn: &mut Connection,
    people_id: u64,
    limit: u64,
    offset: u64,
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
            people_id = ?1
        ORDER BY
            id DESC
        LIMIT
            ?2
        OFFSET
            ?3
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read tags by people_id".to_string()),
    };

    let mut tag_iter = match stmt.query_map((people_id, limit, offset), get_entry_from_row) {
        Ok(tag_iter) => tag_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(entry_maybe) = tag_iter.next() {
        if let Ok(tag) = entry_maybe {
            return Ok(Some(tag));
        }
    }

    Ok(None)
}
