use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    /// Username of the collaborator
    username: String,
    /// Role of the collaborator
    role: String,
}

impl Collaborator {
    pub fn new<T: ToString, S: ToString>(username: T, role: S) -> Self {
        Self {
            username: username.to_string(),
            role: role.to_string(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn role(&self) -> &str {
        &self.role
    }
}
