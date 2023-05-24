use super::object::Object;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub object: Object,
    pub data: Vec<Model>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: Object,
    pub created: usize,
    pub owned_by: String,
    pub permission: Vec<Permission>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Permission {
    pub id: String,
    pub object: Object,
    pub created: usize,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}
