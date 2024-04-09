use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Dek {
    _id: ObjectId,
    pub uid: String,
    pub email: String,
    pub dek: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}