pub mod user;
pub mod model;
pub mod item;
pub mod item_type;
pub mod rarity;


#[cfg(test)]
mod tests {
    use crate::models::rarity::Rarity;

    use super::item::Item;
    use super::item_type::ItemType;
    use super::user::User;

    #[test]
    fn user() {
        let user = User::new(111, "test".to_string(), 0, 0, vec![]);

        assert_eq!(user.id(), 111);
        assert_eq!(user.name, "test");
        assert_eq!(user.bank, 0);
        assert_eq!(user.wallet, 0);
        assert_eq!(user.items.len(), 0);
    }

    #[test]
    fn item() {
        let item = Item::new(111, ItemType::Amethyst, 0.5, 111);

        assert_eq!(item.id(), 111);
        assert_eq!(item.type_(), &ItemType::Amethyst);
        assert_eq!(item.owner_id, 111);
    }

    #[test]
    fn item_type() {
        let item_type = ItemType::Amethyst;
        assert_eq!(item_type.name(), "Amethyst");

        let item_type = ItemType::from_i32(0).unwrap();
        assert_eq!(item_type, ItemType::FishingRod);
        assert_eq!(item_type.to_i32(), 0);
        assert_eq!(item_type.name(), "Fishing Rod");
    }

    #[test]
    fn item_rarity() {
        let item_rarity = Rarity::Common;
        assert_eq!(item_rarity.name(), "Common");

        let item_rarity = Rarity::from_int(0).unwrap();
        assert_eq!(item_rarity, Rarity::Common);
        assert_eq!(item_rarity.to_int(), 0);
        assert_eq!(item_rarity.name(), "Common");
    }
}
