pub mod user_factory;
pub mod item_factory;

#[cfg(test)]
mod tests {
    use crate::factories::item_factory::ItemFactory;
    use crate::factories::user_factory::UserFactory;
    use crate::models::item_type::ItemType;

    #[test]
    fn user_factory() {
        let mut user_factory = UserFactory::new();
        let user = user_factory.create("test");

        assert_eq!(user.name, "test");
        assert_eq!(user.bank, 0);
        assert_eq!(user.wallet, 0);
    }

    #[test]
    fn item_factory() {
        let mut item_factory = ItemFactory::new();
        let item = item_factory.create(ItemType::Amethyst, 1);

        assert_eq!(item.type_(), &ItemType::Amethyst);
        assert_eq!(item.owner_id, 1);
    }
}