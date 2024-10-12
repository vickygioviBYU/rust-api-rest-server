use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

// Debuggeable, Serializable and Desearealizable
#[derive(Debug, Serialize, Deserialize)]
// Course model
// In MongoDB, the entry is in that format
pub struct Course {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub code: String,
    pub name: String,
    pub credits: i32,
}