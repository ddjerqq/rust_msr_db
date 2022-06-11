use rand::Rng;

use crate::id::Id;
use crate::models::item::Item;
use crate::models::item_type::ItemType;

pub struct ItemFactory {
    id_generator: Id,
    rng: rand::rngs::ThreadRng,
}

impl ItemFactory {
    pub fn new() -> Self {
        Self {
            id_generator: Id::new(),
            rng: rand::thread_rng()
        }
    }

    /// generate a new user.
    /// param: name; the name of the new user.
    /// this method handles ID creation and default value assignment
    pub fn create(&mut self, type_: ItemType, owner_id: u64) -> Item {
        Item::new(
            self.id_generator.generate(),
            type_,
            self.rng.gen::<f64>(),
            owner_id,
        )
    }
}
