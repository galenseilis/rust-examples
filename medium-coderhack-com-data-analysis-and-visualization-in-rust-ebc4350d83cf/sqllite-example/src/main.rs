extern crate rusqlite;

use rusqlite::{Connection, params};

fn main() {
    // Open or create an SQLite database file named "test.db"
    let conn = Connection::open("test.db").unwrap();

    // Create the 'person' table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            name TEXT,
            age INTEGER
        )",
        [],
    )
    .unwrap();

    // Insert some entries into the 'person' table
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?, ?)",
        params!["John Doe", 25],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO person (name, age) VALUES (?, ?)",
        params!["Alice Smith", 30],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO person (name, age) VALUES (?, ?)",
        params!["Bob Johnson", 22],
    )
    .unwrap();

    println!("Entries added to the 'person' table.");

    // Query and print entries from the 'person' table
    println!("Querying entries from the 'person' table:");
    let mut stmt = conn.prepare("SELECT name, age FROM person").unwrap();
    let person_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    });

    for person in person_iter.unwrap() {
        let person = person.unwrap();
        println!("Name: {}, Age: {}", person.0, person.1);
    }
}

