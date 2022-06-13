use std::sync::{Arc, Mutex};

use crate::models::model::Model;

pub trait Service<TEntity>
where TEntity: Model
{
    /// initialize a service with the connection
    fn new(connection: Arc<Mutex<rusqlite::Connection>>) -> Self;

    /// get all entities
    fn get_all(&self)                     -> Result<Vec<TEntity>, String>;

    /// get a single entity by id
    fn get_by_id(&self,  id: &u64)         -> Result<Option<TEntity>, String>;

    /// add an entity to the database
    fn add(&mut self,    entity: &TEntity) -> Result<(), String>;

    /// update an entity
    fn update(&mut self, entity: &TEntity) -> Result<(), String>;

    /// delete an entity by its id
    fn delete(&mut self, id: &u64)          -> Result<(), String>;
}