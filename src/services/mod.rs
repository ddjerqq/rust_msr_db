pub mod service;
pub mod user_service;


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::services::service::Service;
    use crate::services::user_service::UserService;

    #[test]
    fn user_service() {
        let con = rusqlite::Connection::open_in_memory().unwrap();
        let _user_service = UserService::new(Arc::new(Mutex::new(con)));
    }
}
