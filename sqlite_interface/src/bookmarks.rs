use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::bookmarks::Bookmark;

fn get_entry_from_row(row: &Row) -> Result<Bookmark, RusqliteError> {
    Ok(Bookmark {
        id: row.get(0)?,
        url: row.get(1)?,
        people_id: row.get(2)?,
        deleted_at: row.get(3)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            id INTEGER PRIMARY KEY,
            url TEXT NOT NULL UNIQUE,
            people_id INTEGER NOT NULL,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(_e) = results {
        return Err("failed to create bookmarks table".to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    people_id: u64,
    url: &str,
) -> Result<Option<Bookmark>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO bookmarks
            (id, url. people_id)
        VALUES
            (?1, ?2, ?3)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement to create bookmark".to_string()),
    };

    let mut entry_iter = match stmt.query_map((id, url, people_id), get_entry_from_row) {
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

pub fn read(conn: &mut Connection, limit: u64, offset: u64) -> Result<Vec<Bookmark>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmarks
        WHERE
            deleted_at IS NULL
        LIMIT
            ?1
        OFFSET
            ?2
        ORDER BY
            id DESC",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read bookmark".to_string()),
    };

    let mut entry_iter = match stmt.query_map((limit, offset), get_entry_from_row) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(e.to_string()),
    };

    let mut bookmarks: Vec<Bookmark> = Vec::new();
    while let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            bookmarks.push(entry);
        }
    }

    Ok(bookmarks)
}

pub fn read_by_id(conn: &mut Connection, id: u64) -> Result<Option<Bookmark>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmarks
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ORDER BY
            id DESC
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare read_by_id statement".to_string()),
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
) -> Result<Option<Bookmark>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmarks
        WHERE
            deleted_at IS NULL
            AND
            people_id = ?1
        LIMIT
            ?2
        OFFSET
            ?3
        ORDER BY
            id DESC
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare read_by_people_id statement".to_string()),
    };

    let mut entry_iter = match stmt.query_map((people_id, limit, offset), get_entry_from_row) {
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
