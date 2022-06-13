use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use rusqlite::{Connection, params};

use crate::models::item::Item;
pub use crate::models::model::Model;
use crate::repositories::repository::Repository;

pub struct ItemRepository {
    connection: Arc<Mutex<Connection>>,
}

impl Repository<Item> for ItemRepository {
    fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    fn save_changes(&mut self) -> Result<(), String> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .borrow_mut()
            .transaction()
            .and_then(|transaction| transaction.commit())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<Item>, String> {
        let mut con =
            self.connection
                .lock()
                .map_err(|e| e.to_string())?;

        let mut stmt = con
            .borrow_mut()
            .prepare("SELECT * FROM items")
            .map_err(|e| e.to_string())?;

        let entities = stmt.query_map([], |row| {
            if let Ok(entity) = Item::from_row(row) {
                Ok(entity)
            } else {
                Err(rusqlite::Error::QueryReturnedNoRows)
            }
        }).map_err(|e| e.to_string())?;

        Ok(entities.flatten().collect())
    }

    fn get_by_id(&self, id: &u64) -> Result<Option<Item>, String> {
        let mut con = self.connection
            .lock()
            .map_err(|e| e.to_string())?;

        let mut stmt = con
            .borrow_mut()
            .prepare("SELECT * FROM items WHERE id=?1")
            .map_err(|e| e.to_string())?;

        let mut result = stmt.query_map([id], |row| {
            Ok(Item::from_row(row))
        }).map_err(|e| e.to_string())?;

        if let Some(entity) = result.next() {
            return if let Ok(Ok(entity)) = entity {
                Ok(Some(entity))
            } else {
                Ok(None)
            }
        }
        Ok(None)
    }

    fn add(&mut self, entity: &Item) -> Result<(), String> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .borrow_mut()
            .execute("
            INSERT INTO items
            (
                id,
                type,
                rarity,
                owner_id
            )
            VALUES(?1, ?2, ?3, ?4)",
            params![
                entity.id(),
                entity.type_().to_i32(),
                entity.rarity.to_f64(),
                entity.owner_id
            ])
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn update(&mut self, entity: &Item) -> Result<(), String> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .borrow_mut()
            .execute("
            UPDATE items
            SET type=?2, rarity=?3, owner_id=?4
            WHERE id=?1;",
            params![
                entity.id(),
                entity.type_().to_i32(),
                entity.rarity.to_f64(),
                entity.owner_id
            ])
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn delete(&mut self, id: &u64) -> Result<(), String> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .borrow_mut()
            .execute("DELETE FROM items WHERE id=?1;",
                     params![id])
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}