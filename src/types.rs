use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCode{
    pub title: String,
    pub author: String,
    pub content: String,
    pub lang: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Code{
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub author: String,
    pub content: String,
    pub hash: String,
    pub lang: String
}