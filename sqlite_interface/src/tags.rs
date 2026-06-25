use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::tags::Tag;

// Some odd conversions between u64 and i64 and back again.
// Sqlite apparently converts a string to a number affinity.
// If the number is too big it is a REAL or similar.
// In this case we never go above 64bits.
//
// Takes an i64 with a number affinity and cast to u64.
// Probably more proper to parse a string, but this introduces
// Errors and i am laaazy right now.
//
// But this should be valid?
//

fn get_entry_from_row(row: &Row) -> Result<Tag, RusqliteError> {
    let id_i64: i64 = row.get(0)?;
    let people_id_i64: i64 = row.get(1)?;
    let title: String = row.get(2)?;
    let deleted_at_i64: Option<i64> = row.get(3)?;

    let deleted_at: Option<u64> = match deleted_at_i64 {
        Some(val) => Some(val as u64),
        None => None,
    };

    Ok(Tag {
        id: id_i64 as u64,
        people_id: people_id_i64 as u64,
        title,
        deleted_at,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id UNSIGNED BIG INT PRIMARY KEY,
            people_id UNSIGNED BIT INT NOT NULL,
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
    people_id: u64,
    title: &str,
) -> Result<Option<Tag>, String> {
    // might need to confirm alphanumeric

    let mut stmt = match conn.prepare(
        "
        INSERT INTO tags
            (id, people_id, title)
        VALUES
            (?1, ?2, lower(?3))
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("Could not prepare a tags create statement".to_string()),
    };

    let mut entry_iter = match stmt.query_map(
        (id.to_string(), people_id.to_string(), title.to_string()),
        get_entry_from_row,
    ) {
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

// params
pub fn read(
    conn: &mut Connection,
    limit: u32,
    offset: u32,
    order: &str,
) -> Result<Vec<Tag>, String> {
    // Rather than dangerously compose templates,
    // pick from a pregenerated one. It's copy paste basically.
    // But it should be a little safer than injections.

    let tempalte = match order {
        "ASC" => {
            "
        SELECT
            *
        FROM
            tags
        WHERE
            deleted_at IS NULL
        ORDER BY
            id ASC
        LIMIT
            ?1
        OFFSET
            ?2
        "
        }
        "DESC" => {
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
        "
        }
        _ => return Err("invalid order property given in read statement".to_string()),
    };

    let mut stmt = match conn.prepare(tempalte) {
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
            id = ?1
            AND
            deleted_at IS NULL
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

// params
pub fn read_by_people_id(
    conn: &mut Connection,
    people_id: u64,
    limit: u32,
    offset: u32,
) -> Result<Vec<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            tags
        WHERE
            people_id = ?1
            AND
            deleted_at IS NULL
        ORDER BY
            id DESC
        LIMIT
            ?2
        OFFSET
            ?3
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("could not prepare a tags read statement".to_string()),
    };

    let mut entry_iter =
        match stmt.query_map((people_id.to_string(), limit, offset), get_entry_from_row) {
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

pub fn read_by_title(conn: &mut Connection, title: &str) -> Result<Option<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            tags
        WHERE
            title = ?1
            AND
            deleted_at IS NULL
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

// params
pub fn delete(conn: &mut Connection, id: u64, deleted_at: u64) -> Result<Option<Tag>, String> {
    let mut stmt = match conn.prepare(
        "
        UPDATE
            tags
        SET
            deleted_at = ?1
        WHERE
            id = ?2
        RETURNING
            *
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("could not prepare a tags read_by_title statment".to_string()),
    };

    let mut entry_iter =
        match stmt.query_map([deleted_at.to_string(), id.to_string()], get_entry_from_row) {
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

// add a limit offset
// params
pub fn dangerously_delete_stale_entries(
    conn: &mut Connection,
    now: u32,
    offset: u32,
    limit: u32,
) -> Result<Vec<Tag>, String> {
    // TODO:
    // Sqlite seems to have a LIMIT clause on DELETE.
    // Instead, two queries are used below.
    // Upgrade when possible.

    let mut stmt = match conn.prepare(
        "
        DELETE FROM
            tags
        WHERE id IN (
            SELECT
                id
            FROM 
                tags 
            WHERE
                deleted_at IS NOT NULL
                AND
                deleted_at + ?1 < ?2
            LIMIT
                ?3
        )
        RETURNING
            *
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("could not prepare a tags dangerously delete statment".to_string()),
    };

    let mut entry_iter = match stmt.query_map(
        [offset.to_string(), now.to_string(), limit.to_string()],
        get_entry_from_row,
    ) {
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
