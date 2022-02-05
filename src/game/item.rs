use serde::Deserialize;

#[derive(Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub hidden: Option<bool>
}