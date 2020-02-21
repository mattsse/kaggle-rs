#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Collaborator {
    /// Username of the collaborator
    #[serde(rename = "username")]
    username: String,
    /// Role of the collaborator
    #[serde(rename = "role")]
    role: String,
}

impl Collaborator {
    pub fn new(username: String, role: String) -> Collaborator {
        Collaborator {
            username: username,
            role: role,
        }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn with_username(mut self, username: String) -> Collaborator {
        self.username = username;
        self
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn set_role(&mut self, role: String) {
        self.role = role;
    }

    pub fn with_role(mut self, role: String) -> Collaborator {
        self.role = role;
        self
    }

    pub fn role(&self) -> &String {
        &self.role
    }
}
