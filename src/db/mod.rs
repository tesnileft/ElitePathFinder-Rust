use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

pub mod sql;

// Each `M::up(...)` is one migration, applied in order
fn migrations() -> Migrations<'static> {
    Migrations::new(vec![M::up(include_str!(
        "../../sql/migrations/0001_initial.sql"
    ))])
}

pub fn open(path: &str) -> Connection {
    let mut conn = Connection::open(path).expect("failed to open database");
    conn.pragma_update(None, "foreign_keys", true)
        .expect("failed to enable foreign key enforcement");
    migrations()
        .to_latest(&mut conn)
        .expect("failed to run database migrations");
    conn
}