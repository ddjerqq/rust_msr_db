#[cfg(test)]
/// NOTES
/// we can always unwrap Connection::open, as it will only fail if the user passes a retarded string
mod tests {
    use msrdbsqlite::database::Database;
    use msrdbsqlite::models::user::User;
    use msrdbsqlite::services::service::Service;


    #[test]
    fn test_database_works() {
        let con = rusqlite::Connection::open(r"C:\work\rust\msrdbsqlite\database.db").unwrap();

        let mut stmt = con.prepare("SELECT * FROM users").unwrap();

        let x = stmt.query_map([], |row| {
            let id: u64 = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let wallet: u64 = row.get(2).unwrap();
            let bank: u64 = row.get(3).unwrap();
            Ok((id, name, wallet, bank))
        }).unwrap();

        for y in x {
            println!("{:?}", y.unwrap());
        }
    }


    #[test]
    fn test_create() {
        let mut db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
        let user = User::new(111, String::from("ddjerqq"), 100, 0);
        db.users.add(&user).unwrap();
    }


    #[test]
    fn test_update() {
        let mut db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
        let mut user = db.users.get_by_id(111).unwrap().unwrap();
        user.wallet += 1000;
        db.users.update(&user).unwrap();
    }

    #[test]
    fn test_delete() {
        let mut db = Database::new(r"C:\work\rust\msrdbsqlite\database.db");
        let user = db.users.delete(125).unwrap();
    }

    #[test]
    fn test_read_returns_not_found() {
        let con = rusqlite::Connection::open(r"C:\work\rust\msrdbsqlite\database.db").unwrap();
        let mut stmt = con.prepare("SELECT * FROM users WHERE id=400").unwrap();

        let users: Vec<(u64, String, u64, u64)> = stmt.query_map([], |row| {
            let id: u64 = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let wallet: u64 = row.get(2).unwrap();
            let bank: u64 = row.get(3).unwrap();

            Ok((id, name, wallet, bank))
        }).unwrap().map(|item| item.unwrap()).collect();

        println!("{:?}", users);
    }

    #[test]
    fn test_connection_error() {
        let con = rusqlite::Connection::open("bruh.db").unwrap();
    }

    #[test]
    fn test_no_table_error() {
        let con = rusqlite::Connection::open("bruh.db").unwrap();
        match con.execute("SELECT * FROM bruh", []) {
            Ok(_) => println!(),
            Err(err) => println!("{}", err),
        }
    }
}
