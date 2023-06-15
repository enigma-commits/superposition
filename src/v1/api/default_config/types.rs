use serde::Deserialize;
use serde_json::{Value, Map};

#[derive(Deserialize)]
pub struct CreateReq {
    pub value: Value,
    pub schema: Map<String, Value>
}
