#[cfg(test)]
mod tests {
    use msrdbsqlite::database::Database;
    use msrdbsqlite::factories::item_factory::ItemFactory;
    use msrdbsqlite::factories::user_factory::UserFactory;
    use msrdbsqlite::models::item_type::ItemType;
    use msrdbsqlite::services::service::Service;

    const DB_PATH: &str = r"C:\work\rust\msrdbsqlite\database.db";

    #[test]
    fn create() {
        let mut db = Database::new(DB_PATH);
        let mut user_factory = UserFactory::new();
        let mut item_factory = ItemFactory::new();

        let mut user = user_factory.create("ddjerqq");
        user.items.extend(vec![
            item_factory.create(ItemType::Amethyst, user.id()),
            item_factory.create(ItemType::CopperCoin, user.id()),
            item_factory.create(ItemType::Diamond, user.id()),
            item_factory.create(ItemType::Emerald, user.id()),
            item_factory.create(ItemType::Deer, user.id()),
        ]);

        db.users.add(&user).unwrap();
    }

    #[test]
    fn read() {
        let db = Database::new(DB_PATH);
        let user = db.users.get_by_id(&985169472044138496).unwrap().unwrap();

        println!("{:?}", user);
    }

    #[test]
    fn update() {
        let mut db = Database::new(DB_PATH);
        let mut user = db.users.get_by_id(&985169472044138496).unwrap().unwrap();

        user.name = "changed".to_string();
        user.items.pop();

        db.users.update(&user).unwrap();
    }

    #[test]
    fn delete() {
        let mut db = Database::new(DB_PATH);
        let user = UserFactory::new().create("test");

        db.users.add(&user).unwrap();

        db.users.delete(&user.id()).unwrap();
    }
}