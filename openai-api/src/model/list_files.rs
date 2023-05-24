use super::object::Object;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub data: Vec<File>,
    pub object: Object,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub id: String,
    pub object: Object,
    pub bytes: usize,
    pub created_at: usize,
    pub filename: String,
    pub purpose: String,
}
