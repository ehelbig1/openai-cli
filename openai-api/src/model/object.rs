use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Object {
    #[serde(rename = "list")]
    List,

    #[serde(rename = "model")]
    Model,

    #[serde(rename = "model_permission")]
    ModelPermission,

    #[serde(rename = "text_completion")]
    TextCompletion,

    #[serde(rename = "chat.completion")]
    ChatCompletion,

    #[serde(rename = "file")]
    File,
}
