use std::sync::{Arc, Mutex};

use crate::services::service::Service;
use crate::services::user_service::UserService;

/// Model Service Repository Pattern Database struct.
///
/// This struct is a `manager` for the services.
pub struct Database {
    pub users:  UserService,
}

impl Database {
    /// Creates a new database instance.
    ///
    /// # Arguments
    ///
    /// * `database_path` - The path to the database file.
    ///
    /// # Examples
    ///
    /// ```
    /// use msrdbsqlite::database::Database;
    /// use msrdbsqlite::factories::user_factory::UserFactory;
    /// use msrdbsqlite::services::service::Service;
    /// // initialize the database
    /// let mut db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
    /// let mut user_factory = UserFactory::new();
    ///
    /// // create
    /// let user = user_factory.create("user_name");
    /// db.users.add(&user).unwrap();
    ///
    /// // read
    /// let mut user = db.users.get_by_id(&user.id()).unwrap().unwrap();
    ///
    /// // update
    /// user.name = "new_name".to_string();
    /// db.users.update(&user).unwrap();
    ///
    /// // delete
    /// db.users.delete(&user.id()).unwrap();
    /// ```
    /// # Returns
    ///
    /// * `Database` - The database instance.
    ///
    /// # Panics
    ///
    /// * `&str` - The database path is invalid.
    ///
    /// # Errors
    ///
    /// * `&str` - The database path is invalid.
    ///
    /// # Notes
    ///
    /// * The database path is relative to the current working directory.
    /// * Or you can use an absolute path.
    ///
    pub fn new(database_path: &str) -> Self {
        let connection =
            Arc::new(
                Mutex::new(
                    rusqlite::Connection::open(database_path).unwrap()
                )
            );

        let users = UserService::new(connection.clone());

        Database { users }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_database_new() {
        Database::new(r"C:\work\rust\msrdbsqlite\database.db");
    }
}
