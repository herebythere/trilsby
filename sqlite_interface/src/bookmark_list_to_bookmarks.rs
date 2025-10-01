use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::bookmark_lists::BookmarkListToBookmark;

fn get_entry_from_row(row: &Row) -> Result<BookmarkListToBookmark, RusqliteError> {
    Ok(BookmarkListToBookmark {
        id: row.get(0)?,
        bookmark_list_id: row.get(1)?,
        bookmark_id: row.get(2)?,
        order_weight: row.get(3)?,
        people_id: row.get(4)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmark_list_to_bookmark (
            id INTEGER PRIMARY KEY,
            bookmark_list_id INTEGER NOT NULL,
            bookmark_id INTEGER NOT NULL,
            order_weight INTEGER NOT NULL,
            people_id INTEGER NOT NULL,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(_e) = results {
        return Err("Failed to create bookmark_list_to_bookmark table.".to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    bookmark_list_id: u64,
    bookmark_id: u64,
    order_weight: u64,
    people_id: u64,
) -> Result<Option<BookmarkListToBookmark>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO bookmark_list_to_bookmark
            (id, people_id, bookmark_list_id, bookmark_id, order_weight)
        VALUES
            (?1, ?2, ?3, ?4, ?5)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => {
            return Err("cound not prepare statement to create BookmarkListToBookmark".to_string())
        }
    };

    let mut entry_iter = match stmt.query_map(
        (id, people_id, bookmark_list_id, bookmark_id, order_weight),
        get_entry_from_row,
    ) {
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

pub fn read(
    conn: &mut Connection,
    limit: u64,
    offset: u64,
) -> Result<Vec<BookmarkListToBookmark>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmark_list_to_bookmark
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
        _ => return Err("failed to read a contact".to_string()),
    };

    let mut list_to_bookmark = match stmt.query_map((limit, offset), get_entry_from_row) {
        Ok(list_to_bookmark) => list_to_bookmark,
        Err(e) => return Err(e.to_string()),
    };

    let mut list_to_bookmarks: Vec<BookmarkListToBookmark> = Vec::new();
    while let Some(entry_maybe) = list_to_bookmark.next() {
        if let Ok(entry) = entry_maybe {
            list_to_bookmarks.push(entry);
        }
    }

    Ok(list_to_bookmarks)
}

pub fn read_by_people_id(
    conn: &mut Connection,
    people_id: &str,
    limit: u64,
    offset: u64,
) -> Result<Option<BookmarkListToBookmark>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmark_list_to_bookmark
        WHERE
            deleted_at IS NULL
            AND
            people_id = ?1
        ORDER BY
            id DESC, order_weight DESC, bookmark_list DESC
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

pub fn read_by_bookmark_list_id(
    conn: &mut Connection,
    bookmark_list_id: &str,
    limit: u64,
    offset: u64,
) -> Result<Option<BookmarkListToBookmark>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            bookmark_list_to_bookmark
        WHERE
            deleted_at IS NULL
            AND
            bookmark_list_id = ?1
        LIMIT
            ?2
        OFFSET
            ?3
        ORDER BY
            id DESC, order_weight DESC
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare read_by_bookmark_list_id statement".to_string()),
    };

    let mut entry_iter = match stmt.query_map((bookmark_list_id, limit, offset), get_entry_from_row)
    {
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
