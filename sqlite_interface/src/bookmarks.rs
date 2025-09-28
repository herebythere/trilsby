use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::bookmarks::Bookmark;

fn get_bookmark_from_row(row: &Row) -> Result<Bookmark, RusqliteError> {
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

    if let Err(e) = results {
        return Err("bookmarks table error: \n".to_string() + &e.to_string());
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
        _ => return Err("cound not prepare create statement".to_string()),
    };

    let mut bookmark_iter = match stmt.query_map((id, url, people_id), get_bookmark_from_row) {
        Ok(bookmark_iter) => bookmark_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(bookmark_maybe) = bookmark_iter.next() {
        if let Ok(bookmark) = bookmark_maybe {
            return Ok(Some(bookmark));
        }
    }

    Ok(None)
}

// read all (connection, limit, offset)

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
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare read_by_id statement".to_string()),
    };

    let mut bookmark_iter = match stmt.query_map([id], get_bookmark_from_row) {
        Ok(bookmark) => bookmark,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(bookmark_maybe) = bookmark_iter.next() {
        if let Ok(bookmark) = bookmark_maybe {
            return Ok(Some(bookmark));
        }
    }

    Ok(None)
}

pub fn read_by_people_id(
    conn: &mut Connection,
    people_id: u64,
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
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare read_by_people_id statement".to_string()),
    };

    let mut bookmark_iter = match stmt.query_map([people_id], get_bookmark_from_row) {
        Ok(bookmark_iter) => bookmark_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(bookmark_maybe) = bookmark_iter.next() {
        if let Ok(bookmark) = bookmark_maybe {
            return Ok(Some(bookmark));
        }
    }

    Ok(None)
}
