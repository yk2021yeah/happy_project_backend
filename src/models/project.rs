use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Projects {
    #[serde(skip_deserializing)]
    pub _id: ObjectId,
    pub project_name: String,
    pub project_owner_id: u64,
    pub start_date: String,
    pub end_date: String,
    pub project_member_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    #[serde(skip_deserializing)]
    pub _id: ObjectId,
    pub user_name: String,
    pub created: String,
    pub updated: String,
}
