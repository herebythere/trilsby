use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::BookmarkList;

fn get_entry_from_row(row: &Row) -> Result<BookmarkList, RusqliteError> {
    Ok(BookmarkList {
        id: row.get(0)?,
        people_id: row.get(1)?,
        deleted_at: row.get(2)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmark_lists (
            id INTEGER PRIMARY KEY,
            people_id INTEGER NOT NULL,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("bookmark_list table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    people_id: u64,
) -> Result<Option<BookmarkList>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO bookmark_lists
            (id, people_id)
        VALUES
            (?1, ?2)
        RETURNING
            *
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare create statement".to_string()),
    };

    let mut entry_iter = match stmt.query_map((id, people_id), get_entry_from_row) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, limit: u64, offset: u64) -> Result<Vec<BookmarkList>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmark_lists
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
        _ => return Err("failed to read a bookmark_list".to_string()),
    };

    let mut entry_iter = match stmt.query_map((limit, offset), get_entry_from_row) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(e.to_string()),
    };

    let mut bookmark_lists: Vec<BookmarkList> = Vec::new();
    while let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            bookmark_lists.push(entry);
        }
    }

    Ok(bookmark_lists)
}

pub fn read_by_id(conn: &mut Connection, id: u64) -> Result<Option<BookmarkList>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmark_lists
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("could not prepare read statement".to_string()),
    };

    let mut entry_iter = match stmt.query_map([id], get_entry_from_row) {
        Ok(bookmark) => bookmark,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

pub fn read_by_people_id(
    conn: &mut Connection,
    people_id: u64,
    limit: u64,
    offset: u64,
) -> Result<Vec<BookmarkList>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmark_lists
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
        _ => return Err("cound not prepare read_by_people_id statement".to_string()),
    };

    let mut entry_iter = match stmt.query_map((people_id, limit, offset), get_entry_from_row) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(e.to_string()),
    };

    let mut bookmark_lists: Vec<BookmarkList> = Vec::new();
    if let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            bookmark_lists.push(entry);
        }
    }

    Ok(bookmark_lists)
}
