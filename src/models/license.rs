use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    /// Name of the license
    name: String,
}

impl License {
    pub fn new<T: ToString>(name: T) -> License {
        License {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
