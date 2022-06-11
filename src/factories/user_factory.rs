use crate::id::Id;
use crate::models::user::User;

pub struct UserFactory {
    id_generator: Id,
}

impl UserFactory {
    pub fn new() -> Self {
        Self { id_generator: Id::new() }
    }

    /// generate a new user.
    /// param: name; the name of the new user.
    /// this method handles ID creation and default value assignment
    pub fn create(&mut self, name: &str) -> User {
        User::new(
           self.id_generator.generate(),
            name.to_string(),
            0,
            0,
            Vec::new()
        )
    }
}
