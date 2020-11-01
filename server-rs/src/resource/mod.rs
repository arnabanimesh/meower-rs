use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;
use chrono::{DateTime,Local};

use crate::common::*;
pub use controller::*;
pub use service::*;

mod controller;
mod service;

impl Resource {
    pub const COLLECTION_NAME: &'static str = "mews";
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Resource{
    #[serde(serialize_with = "serialize_object_id", rename = "_id")]
    id: Option<ObjectId>,
    name: String,
    content: String,
    #[serde(serialize_with = "serialize_date")]
    created: Option<DateTime<Local>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResourceQuery {
    #[serde(default)]
    keyword: String,
}