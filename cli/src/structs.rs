use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde;


#[derive(Serialize, Deserialize)]
pub struct ProjectResponse {
    pub name: String,
    pub description: Option<String>,
    pub members: Option<Vec<String>>,
    pub data: String,
    pub owner: String,
    pub date_created: String,
    pub date_modified: String,
}

pub type ListProjectResponse = Vec<ListProjectResponse1>;

#[derive(Serialize, Deserialize)]
pub struct ListProjectResponse1 {
    pub name: String,
    pub description: String,
    pub members: Vec<Value>,
    pub data: String,
    pub owner: String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFile {
    pub name: String,
    pub file: Option<PathBuf>,
    pub description: String,
    pub environments: Vec<String>,
    pub selected_environment: String,
}

