use anyhow::Result;
use rusqlite::Connection;

pub fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS servers (
            server_id INTEGER PRIMARY KEY AUTOINCREMENT,
            boot INTEGER NOT NULL DEFAULT 1
        )",
        [],
    )?;
    Ok(())
}
