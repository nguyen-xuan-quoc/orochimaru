use crate::keyring::{ActiveModel, Column, Entity, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

/// Keyring table
pub struct KeyringTable<'a> {
    /// Database connection
    pub connection: &'a DatabaseConnection,
}

impl<'a> KeyringTable<'a> {
    /// Create new instance of keyring table
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }

    /// Find keyring record by its id
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(self.connection).await
    }

    /// Find keyring record by its name
    pub async fn find_by_name(&self, name: String) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Username.eq(name))
            .one(self.connection)
            .await
    }

    /// Get all keys in keyring table
    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(self.connection).await
    }

    /// Insert data to keyring table
    pub async fn insert(&self, json_record: serde_json::Value) -> Result<Model, DbErr> {
        let new_record = ActiveModel::from_json(json_record)?;
        Entity::insert(new_record)
            .exec_with_returning(self.connection)
            .await
    }
}
