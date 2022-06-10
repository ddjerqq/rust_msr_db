use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::models::user::User;
use crate::repositories::repository::Repository;
use crate::repositories::user_repository::UserRepository;
use crate::services::service::Service;

pub struct UserService {
    users: UserRepository
}

impl Service<User> for UserService {
    fn new(connection: Arc<Mutex<Connection>>) -> Self {
        let user_repo = UserRepository::new(connection.clone());

        Self {
            users: user_repo,
        }
    }

    fn get_all(&self) -> Result<Vec<User>, String> {
        self.users.get_all()
    }

    fn get_by_id(&self, id: u64) -> Result<Option<User>, String> {
        self.users.get_by_id(id)
    }

    fn add(&mut self, entity: &User) -> Result<(), String> {
        match self.users.add(entity) {
            Ok(_) => {
                self.users.save_changes()?;
                Ok(())
            },
            Err(msg) => Err(msg),
        }
    }

    fn update(&mut self, entity: &User) -> Result<(), String> {
        match self.users.update(entity) {
            Ok(_) => {
                self.users.save_changes()?;
                Ok(()) 
            },
            Err(msg) => Err(msg),
        }
    }

    fn delete(&mut self, id: u64) -> Result<(), String> {
        match self.users.delete(id) {
            Ok(_) => {
                self.users.save_changes()?;
                Ok(())
            },
            Err(msg) => Err(msg),
        }
    }
}
