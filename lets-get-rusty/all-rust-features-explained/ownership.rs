/*
Ownership rules
1. Each value in Rust has a variable that's called its owner.
2. There can be only one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/

struct DatabaseConnection {
	connection: Connection,
}

impl DatabaseConnection {
	fn new(db_name: &str) -> Result<DatabaseConnection, Error> {
		let connection = Connection::open(db_name)?;
		OK(DatabaseConnection {connection})
	}
}

fn main() -> Result<(), Error> {
	let connection = DatabaseConnection::new("example.db")?;
	Ok(())
}
