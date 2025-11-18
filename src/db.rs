use tokio_rusqlite::Connection;

pub async fn initialize_database(conn: &Connection) -> Result<(), tokio_rusqlite::Error> {
    conn.call(|conn| {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );
            CREATE TABLE IF NOT EXISTS channels (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                parent_id INTEGER,
                FOREIGN KEY(parent_id) REFERENCES channels(id)
            );",
        )?;
        Ok(())
    }).await
}
