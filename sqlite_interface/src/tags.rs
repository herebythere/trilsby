use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::tags::Tag;

// Some odd conversions between u64 and i64 and back again.
// Sqlite apparently converts a string to a number affinity.
// If the number is too big it is a REAL or similar.
// In this case we never go above 64bits.
//
// Takes an i64 witha number affinity and
//

fn get_entry_from_row(row: &Row) -> Result<Tag, RusqliteError> {
    let id_i64: i64 = row.get(0)?;
    let deleted_at_i64: Option<i64> = row.get(2)?;

    let id: u64 = id_i64 as u64;
    let deleted_at: Option<u64> = match deleted_at_i64 {
        Some(val) => Some(val as u64),
        None => None,
    };

    Ok(Tag {
        id,
        title: row.get(1)?,
        deleted_at,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id UNSIGNED BIG INT PRIMARY KEY,
            title TEXT NOT NULL UNIQUE,
            deleted_at UNSIGNED BIG INT
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
    // people_id: u64,
    title: &str,
) -> Result<Option<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO tags
            (id, title)
        VALUES
            (?1, ?2)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("Could not prepare a tags create statement".to_string()),
    };

    let mut entry_iter =
        match stmt.query_map((id.to_string(), title.to_string()), get_entry_from_row) {
            Ok(entry_iter) => entry_iter,
            Err(e) => return Err(e.to_string()),
        };

    if let Some(entry_maybe) = entry_iter.next() {
        println!("{:?}", &entry_maybe);

        if let Ok(entry) = entry_maybe {
            println!("found something! {:?}", &entry);
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, limit: u32, offset: u32) -> Result<Vec<Tag>, String> {
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
        _ => return Err("could not prepare a tags read statement".to_string()),
    };

    let mut entry_iter = match stmt.query_map((limit, offset), get_entry_from_row) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(e.to_string()),
    };

    let mut tags: Vec<Tag> = Vec::new();
    while let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            tags.push(entry);
        }
    }

    Ok(tags)
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
        _ => return Err("could not prepare a tags read_by_id statement".to_string()),
    };

    let mut entry_iter = match stmt.query_map([id.to_string()], get_entry_from_row) {
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

pub fn read_by_title(conn: &mut Connection, title: &str) -> Result<Option<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            tags
        WHERE
            deleted_at IS NULL
            AND
            title = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("could not prepare a tags read_by_title statment".to_string()),
    };

    let mut entry_iter = match stmt.query_map([title], get_entry_from_row) {
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
