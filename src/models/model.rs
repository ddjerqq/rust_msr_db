use rusqlite::ToSql;

pub trait Model {
    fn new(row: &rusqlite::Row) -> Result<Self, String>
    where
        Self: Sized;

    fn params(&self) -> Vec<&dyn ToSql>;
}