#[cfg(test)]
mod msr_test {
    use msrdbsqlite::database::Database;
    use msrdbsqlite::models::user::User;
    use msrdbsqlite::services::service::Service;

    #[test]
    fn test_get_all() {
        let db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
        let users = db.users.get_all().unwrap();
        println!("{:?}", users);
    }

    #[test]
    fn test_get_by_id() {
        let db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
        let user = db.users.get_by_id(111).unwrap().unwrap();
        let test = User::new(111, "ddjerqq".to_string(), 4100, 0);

        assert_eq!(user.id, test.id);
        assert_eq!(user.name, test.name);
        assert_eq!(user.wallet, test.wallet);
        assert_eq!(user.bank, test.bank);
    }

    #[test]
    fn test_add() {
        let mut db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
        let user = User::new(123, "test".to_string(), 20, 0);
        db.users.add(&user).unwrap();

        let test = db.users.get_by_id(123).unwrap().unwrap();

        assert_eq!(user.id, test.id);
        assert_eq!(user.name, test.name);
        assert_eq!(user.wallet, test.wallet);
        assert_eq!(user.bank, test.bank);
    }

    #[test]
    fn test_update() {
        let mut db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
        let mut user = User::new(125, "test".to_string(), 20, 0);
        db.users.add(&user).unwrap();

        {
            let test = db.users.get_by_id(125).unwrap().unwrap();

            assert_eq!(user.id, test.id);
            assert_eq!(user.name, test.name);
            assert_eq!(user.wallet, test.wallet);
            assert_eq!(user.bank, test.bank);
        }

        user.name = "bruh moment".to_string();
        db.users.update(&user).unwrap();

        {
            let test = db.users.get_by_id(125).unwrap().unwrap();

            assert_eq!(user.id, test.id);
            assert_eq!(user.name, test.name);
            assert_eq!(user.wallet, test.wallet);
            assert_eq!(user.bank, test.bank);
        }
    }

    #[test]
    fn test_delete() {
        let mut db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
        let user = User::new(126, "test".to_string(), 20, 0);
        db.users.add(&user).unwrap();

        {
            let test = db.users.get_by_id(126).unwrap().unwrap();

            assert_eq!(user.id, test.id);
            assert_eq!(user.name, test.name);
            assert_eq!(user.wallet, test.wallet);
            assert_eq!(user.bank, test.bank);
        }

        db.users.delete(126).unwrap();

        let test = db.users.get_by_id(126).unwrap();
        assert!(test.is_none());
    }
}