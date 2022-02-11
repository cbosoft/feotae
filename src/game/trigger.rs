use serde::Deserialize;

#[derive(Deserialize)]
pub struct Trigger {
    pub payload: String
}