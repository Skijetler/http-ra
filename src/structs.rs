use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "RequestType", rename_all = "PascalCase")]
pub enum Request {
    Get  { url: String },
    Post { url: String, body_type: RequestBodyType, body: HashMap<String, String>},
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum RequestBodyType {
    Form,
    Json
}