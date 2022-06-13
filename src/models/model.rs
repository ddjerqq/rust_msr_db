pub trait Model
where Self: PartialEq
{
    fn from_row(row: &rusqlite::Row) -> Result<Self, String>
    where
        Self: Sized;
}