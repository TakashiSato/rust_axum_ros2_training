use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct User {
    id: u64,
    username: String,
}

impl User {
    pub fn new(id: u64, username: String) -> User {
        User { id, username }
    }

    pub fn to_msg(&self) -> r2r::std_msgs::msg::String {
        r2r::std_msgs::msg::String {
            data: self.username.clone(),
        }
    }
}
