use app::{MainDatabase, PgPooledConnection};
use rand;
use rocket::local::blocking::Client;
use std::time::SystemTime;

use db::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

fn run_sql(query: &str) {
    // Establish a connection to the default database
    let template: ConnectionManager<PgConnection> =
        ConnectionManager::new("postgres://localhost/postgres");
    let pool = Pool::builder()
        .max_size(1)
        .connection_timeout(std::time::Duration::from_millis(100))
        .build(template)
        .unwrap();
    let conn = pool.get().unwrap();
    // Execute the given query
    use db::RunQueryDsl;
    diesel::sql_query(query)
        .execute(&conn)
        .expect(&format!("Query failed: {}", query));
}

/// Stores a unique, thread-safe mock environment to be used within tests.
pub struct MockEnvironment {
    db_name: String,
    client: Option<Client>,
}

impl MockEnvironment {
    /// Generates a temporary database, returning a unique mock environment for testing.
    pub fn new() -> Self {
        // Generate a name for the temporary database based on the current time
        let t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let db_name = format!("zero2prod_{}_{}", t.as_secs(), rand::random::<u8>());

        // Create the temporary database
        run_sql(&format!("CREATE DATABASE {} TEMPLATE zero2prod", db_name));

        let db_url = format!("postgres://localhost/{}", db_name);
        MockEnvironment {
            db_name,
            client: Some(Client::tracked(app::app(&db_url)).expect("valid rocket instance")),
        }
    }
    /// Returns a blocking client to the rocket application under test.
    pub fn client(&mut self) -> &mut Client {
        self.client.as_mut().unwrap()
    }
    #[allow(dead_code)]
    pub fn connection(&mut self) -> PgPooledConnection {
        let pool = self.client().rocket().state::<MainDatabase>().unwrap();
        pool.get().unwrap()
    }
}

impl Drop for MockEnvironment {
    fn drop(&mut self) {
        // Drop the client first to disconnect from the DB
        self.client = None;
        // Then physically drop the testing database
        run_sql(&format!("DROP DATABASE IF EXISTS {}", self.db_name));
    }
}
