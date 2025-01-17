use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::path;
use std::path::PathBuf;
use tokio::fs;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub sqlite_auth_db: PathBuf,
}

pub async fn from_filepath(filepath: &PathBuf) -> Result<Config, String> {
    // get position relative to working directory
    let curr_dir = match env::current_dir() {
        Ok(d) => d,
        _ => return Err("parent directory of config not found".to_string()),
    };

    let config_path = match path::absolute(curr_dir.join(filepath)) {
        Ok(pb) => pb,
        Err(e) => return Err(e.to_string()),
    };

    let parent_dir = match config_path.parent() {
        Some(p) => p.to_path_buf(),
        _ => return Err("parent directory of config not found".to_string()),
    };

    let json_as_str = match fs::read_to_string(&config_path).await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    let config: Config = match serde_json::from_str(&json_as_str) {
        Ok(j) => j,
        Err(e) => return Err(e.to_string()),
    };

    let sqlite_auth_db = match path::absolute(parent_dir.join(&config.sqlite_auth_db)) {
        Ok(j) => j,
        Err(e) => return Err(e.to_string()),
    };

    if sqlite_auth_db.is_dir() {
        return Err(
            "failed to create absolute path from relative path for sqlite_auth_db".to_string(),
        );
    }

    Ok(Config {
        sqlite_auth_db: sqlite_auth_db,
    })
}
