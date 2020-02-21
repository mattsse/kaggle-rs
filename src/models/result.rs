#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {}

impl Result {
    pub fn new() -> Result {
        Result {}
    }
}
