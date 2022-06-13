use crate::models::item_type::ItemType;
use crate::models::model::Model;
use crate::models::rarity::Rarity;

#[derive(Debug)]
pub struct Item {
    id: u64,
    type_: ItemType,
    pub rarity: Rarity,
    pub owner_id: u64,
}

impl Item {
    pub fn id(&self) -> u64 {
        self.id
    }
    
    pub fn type_(&self) -> &ItemType {
        &self.type_
    }
    
    pub fn new(id: u64, type_: ItemType, rarity: f64, owner_id: u64) -> Self {
        Self { id, type_, rarity: Rarity::from_f64(rarity), owner_id }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Model for Item {
    fn from_row(row: &rusqlite::Row) -> Result<Self, String> {
        let id = row.get(0)
            .map_err(|e| e.to_string())?;

        let type_ = ItemType::from_i32(row.get(1)
            .map_err(|e| e.to_string())?)?;

        let rarity = Rarity::from_f64(row.get(2)
            .map_err(|e| e.to_string())?);

        let owner_id = row.get(3)
            .map_err(|e| e.to_string())?;

        Ok(Self { id, type_, rarity, owner_id })
    }
}
