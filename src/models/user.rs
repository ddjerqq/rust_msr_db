use rusqlite::{Statement, ToSql};
use crate::models::model::Model;

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub wallet: u64,
    pub bank: u64,
}

impl Model for User {
    fn new(row: &rusqlite::Row) -> Result<Self, String> {

        let id = row.get(0).map_err(|e| e.to_string())?;
        let name = row.get(1).map_err(|e| e.to_string())?;
        let wallet = row.get(2).map_err(|e| e.to_string())?;
        let bank = row.get(3).map_err(|e| e.to_string())?;

        Ok(Self { id, name, wallet, bank })
    }

    fn params(&self) -> Vec<&dyn ToSql> {
        vec![&self.id, &self.name, &self.wallet, &self.bank]
    }
}
