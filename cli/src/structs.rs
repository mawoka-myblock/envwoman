use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde;


#[derive(Serialize, Deserialize)]
pub struct ProjectResponse {
    pub name: String,
    pub description: Option<String>,
    pub members: Option<Vec<String>>,
    pub environments: Vec<String>,
    pub data: Vec<HashMap<String, String>>,
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
    pub owner: String,
    pub date_created: String,
    pub date_modified: String,
    pub environments: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFile {
    pub name: String,
    pub file: Option<PathBuf>,
    pub description: String,
    pub environments: Vec<String>,
    pub selected_environment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProject {
    pub name: String,
    pub description: String,
    pub environments: Vec<String>,
    pub selected_environment: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(skip_serializing)]
    pub data: Option<String>,
}
