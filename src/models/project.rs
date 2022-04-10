use mongodb::bson::{document::IntoIter, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::iter::IntoIterator;

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

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}