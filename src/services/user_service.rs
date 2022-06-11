use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::models::item::Item;
use crate::models::user::User;
use crate::repositories::item_repository::ItemRepository;
use crate::repositories::repository::Repository;
use crate::repositories::user_repository::UserRepository;
pub use crate::services::service::Service;

pub struct UserService {
    users: UserRepository,
    items: ItemRepository,
}

impl Service<User> for UserService {
    fn new(connection: Arc<Mutex<Connection>>) -> Self {
        let user_repo = UserRepository::new(connection.clone());
        let item_repo = ItemRepository::new(connection);

        Self {
            users: user_repo,
            items: item_repo,
        }
    }

    fn get_all(&self) -> Result<Vec<User>, String> {
        let mut users = self.users.get_all()?;
        let items = self.items.get_all()?;

        let mut items_by_user_id: HashMap<u64, Vec<Item>> = HashMap::new();

        for item in items.into_iter() {
            items_by_user_id.get_mut(&item.owner_id)
                .unwrap_or(&mut Vec::new())
                .push(item);
        }

        for user in users.iter_mut() {
            user.items
                .extend(items_by_user_id.remove(&user.id()).unwrap_or_default());
        }

        Ok(users)
    }

    fn get_by_id(&self, id: &u64) -> Result<Option<User>, String> {
        let user = self.users.get_by_id(id)?;
        if let Some(mut user) = user {
            let items = self.items.get_all()?;
            user.items = items.into_iter().filter(|i| i.owner_id == user.id()).collect();
            return Ok(Some(user));
        }
        Ok(None)
    }

    fn add(&mut self, entity: &User) -> Result<(), String> {
        self.users.add(entity)?;

        for item in entity.items.iter() {
            self.items.add(item)?;
        }

        self.users.save_changes()?;
        self.items.save_changes()?;
        Ok(())
    }

    fn update(&mut self, entity: &User) -> Result<(), String> {
        self.users.update(entity)?;
        let items = self.items.get_all()?;

        for item in entity.items.iter() {
            if items.contains(item) {
                self.items.update(item)?;
            } else {
                self.items.add(item)?;
            }
        };

        for item in items.iter().filter(|i| i.owner_id == entity.id()) {
            if !entity.items.contains(item) {
                self.items.delete(&item.id())?;
            }
        }

        self.users.save_changes()?;
        self.items.save_changes()?;
        Ok(())
    }

    fn delete(&mut self, id: &u64) -> Result<(), String> {
        let entity = self.users.get_by_id(id)?;
        if let Some(entity) = entity {
            for item in entity.items.iter() {
                self.items.delete(&item.id())?;
            }
            self.users.delete(id)?;

            self.users.save_changes()?;
            self.items.save_changes()?;
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }
}
