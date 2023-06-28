use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Parameter,
}

impl Function {
    pub fn new(name: String, description: String) -> Self {
        let parameters = Parameter::Object(JsonObject::new(HashMap::new(), vec![]));

        Self {
            name,
            description,
            parameters,
        }
    }

    pub fn add_property(mut self, key: String, value: Parameter, required: bool) -> Self {
        match &mut self.parameters {
            Parameter::Object(obj) => {
                obj.properties.insert(key.clone(), value);

                if required {
                    obj.required.push(key)
                }
            }
            Parameter::String(_) => unreachable!(),
        }

        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Parameter {
    #[serde(rename = "object")]
    Object(JsonObject),

    #[serde(rename = "string")]
    String(JsonString),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonObject {
    pub properties: HashMap<String, Parameter>,
    pub required: Vec<String>,
}

impl JsonObject {
    pub fn new(properties: HashMap<String, Parameter>, required: Vec<String>) -> Self {
        Self {
            properties,
            required,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonString {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
}

impl JsonString {
    pub fn new(description: Option<String>, r#enum: Option<Vec<String>>) -> Self {
        Self {
            description,
            r#enum,
        }
    }
}
