#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    /// Name of the license
    #[serde(rename = "name")]
    name: String,
}

impl License {
    pub fn new(name: String) -> License {
        License { name }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_name(mut self, name: String) -> License {
        self.name = name;
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
