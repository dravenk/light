use std::fs;

use rusqlite::{Connection, Result};

use light_utils::appdata;
use tracing::debug;

#[derive(Debug)]
pub struct Token {}

fn create_tables(conn: &Connection) -> Result<()> {
    debug!("create_tables...");
    match conn.execute(
        "CREATE TABLE if not exists token (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    ) {
        Ok(_) => debug!("create table token success"),
        Err(e) => debug!("create table token error: {}", e),
    };

    match conn.execute(
        "CREATE TABLE if not exists msg (
            id    INTEGER PRIMARY KEY,
            value TEXT NOT NULL
        )",
        (),
    ) {
        Ok(_) => debug!("create table msg success"),
        Err(e) => debug!("create table msg error: {}", e),
    };

    Ok(())
}

pub fn conn() -> Result<Connection> {
    let result = init_db_dir();
    match result {
        Ok(_) => debug!("init db dir success"),
        Err(e) => debug!("init db dir error: {}", e),
    };
    let conn = Connection::open(appdata::db_path())?;
    Ok(conn)
}

fn init_db_dir() -> std::io::Result<()> {
    fs::create_dir_all(appdata::db_dir())?;
    Ok(())
}

pub fn _insert_value(conn: Connection, msg: &str) -> Result<Connection> {
    match conn.execute("INSERT INTO msg (value) VALUES (?1)", [msg]) {
        Ok(_) => debug!("insert msg success"),
        Err(e) => debug!("insert msg error: {}", e),
    };
    Ok(conn)
}

pub fn _select_table(conn: &Connection, table_name: &str) -> Result<Vec<(i32, String)>> {
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table_name))?;
    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let mut results = Vec::new();
    for row_result in rows {
        results.push(row_result?);
    }

    Ok(results)
}

pub fn sqlite_init() -> Result<Connection> {
    debug!("sqlite_init...");
    let conn = conn().unwrap();
    let _ = create_tables(&conn);
    Ok(conn)
}

pub fn _sqlite_conn() -> Result<Connection> {
    let conn = conn().unwrap();
    Ok(conn)
}
