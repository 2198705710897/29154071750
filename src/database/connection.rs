use rusqlite::Connection;
use anyhow::Result;

pub fn establish_connection(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    Ok(conn)
}
