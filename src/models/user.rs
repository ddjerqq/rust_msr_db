use rusqlite::Row;

use crate::models::item::Item;
use crate::models::model::Model;

#[derive(Debug)]
pub struct User {
    id: u64,
    pub name: String,
    pub wallet: u64,
    pub bank: u64,
    pub items: Vec<Item>,
}

impl User {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn new(id: u64, name: String, wallet: u64, bank: u64, items: Vec<Item>) -> Self {
        Self { id, name, wallet, bank, items }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Model for User {
    fn from_row(row: &Row) -> Result<Self, String> where Self: Sized {
        let id = row.get(0).map_err(|e| e.to_string())?;
        let name = row.get(1).map_err(|e| e.to_string())?;
        let wallet = row.get(2).map_err(|e| e.to_string())?;
        let bank = row.get(3).map_err(|e| e.to_string())?;

        Ok(Self {
            id,
            name,
            wallet,
            bank,
            items: Vec::new(),
        })
    }
}
