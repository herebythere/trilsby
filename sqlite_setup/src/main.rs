use config;
use rusqlite::{params, Connection, Result};
use std::{env, path};

// Trilsby uses a 0 time in mS representing
//  Jan 1st 2025

// So every time field is an INT representing mS since Jan 1st 2025

#[tokio::main]
async fn main() {
    // create config
    let args = match env::args().nth(1) {
        Some(a) => path::PathBuf::from(a),
        None => return println!("argument error: argv[0] config path not provided"),
    };
    let config = match config::from_filepath(&args).await {
        Ok(c) => c,
        Err(e) => return println!("{}", e),
    };

    let auth_conn = match Connection::open(config.sqlite_auth_db) {
        Ok(ac) => ac,
        Err(e) => return println!("{}", e),
    };

    if let Err(e) = auth_conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
			id INTEGER PRIMARY KEY,
			email TEXT NOT NULL UNIQUE,
			password_hash_params TEXT NOT NULL,
			updated_at INTEGER,
			deleted_at INTEGER
		);",
        (),
    ) {
        println!("{}", e);
    };

    if let Err(e) = auth_conn.execute(
        "CREATE TABLE IF NOT EXISTS roles (
			id INTEGER PRIMARY KEY,
			role TEXT NOT NULL,
			belongs_to INTEGER NOT NULL,
			updated_at INTEGER,
			deleted_at INTEGER
		);",
        (),
    ) {
        println!("{}", e);
    };

    if let Err(e) = auth_conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
			id INTEGER PRIMARY KEY,
			session TEXT NOT NULL,
			belongs_to INTEGER NOT NULL,
			role INTEGER NOT NULL,
			session_length_ms INTEGER NOT NULL
		);",
        (),
    ) {
        println!("{}", e);
    };

    if let Err(e) = auth_conn.execute(
        "CREATE TABLE IF NOT EXISTS invitations (
			id INTEGER PRIMARY KEY,
			belongs_to INTEGER NOT NULL,
			claimed_by INTEGER,
			claimed_at INTEGER,
			deleted_at INTEGER
		)",
        (),
    ) {
        println!("{}", e);
    };

    // auth_conn.close();

    // let domain_conn = Connection::open(config.sqlite_domain_db)?;

    // // domain_conn.execute(
    // // 	"CREATE TABLE IF NOT EXISTS profiles (
    // // 		id INTEGER PRIMARY KEY,
    // // 		screen_name TEXT NOT NULL,
    // // 		bio TEXT NOT NULL,
    // // 		profile_image TEXT NOT NULL,
    // // 		belongs_to INTEGER NOT NULL,
    // // 		updated_at INTEGER,
    // // 		deleted_at INTEGER
    // // 	)",
    // //     (),
    // // )?;

    // domain_conn.execute(
    // 	"CREATE TABLE IF NOT EXISTS bookmark_collections (
    // 		id INTEGER PRIMARY KEY,
    // 		title TEXT NOT NULL,
    // 		belongs_to INTEGER NOT NULL,
    // 		updated_at INTEGER,
    // 		deleted_at INTEGER
    // 	)",
    //     (),
    // )?;

    // domain_conn.execute(
    //     "CREATE TABLE IF NOT EXISTS bookmarks (
    // 		id INTEGER PRIMARY KEY,
    // 		url TEXT NOT NULL,
    // 		bookmark_id INTEGER NOT NULL,
    // 		belongs_to INTEGER NOT NULL,
    // 		updated_at INTEGER,
    // 		deleted_at INTEGER
    // 	)",
    //     (),
    // )?;

    // auth_conn.close();
}
