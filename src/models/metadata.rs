use crate::models::License;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub id: String,
    pub licenses: Vec<License>,
    pub resources: Vec<Resource>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub path: String,
    pub description: String,
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
