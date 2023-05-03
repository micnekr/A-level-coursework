use diesel::pg::PgConnection;
use diesel::prelude::*;

/// Establishes a connection with the database using the url to connect to DBMS
pub fn establish_connection(database_url: String) -> PgConnection {
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
