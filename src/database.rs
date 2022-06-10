use std::sync::{Arc, Mutex};
use crate::services::service::Service;
use crate::services::user_service::UserService;

pub struct Database {
    pub users:  UserService,
    connection: Arc<Mutex<rusqlite::Connection>>
}

impl Database {
    // TODO make this return result, in case of idek what, we will have to figure that out too
    pub fn new(database_path: &str) -> Self {
        let connection =
            Arc::new(
                Mutex::new(
                    rusqlite::Connection::open(database_path).unwrap()
                )
            );

        let users = UserService::new(connection.clone());

        Database { users, connection }
    }
}
