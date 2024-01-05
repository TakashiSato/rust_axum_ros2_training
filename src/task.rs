use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub taskname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Task {
    id: u64,
    taskname: String,
}

impl Task {
    pub fn new(id: u64, taskname: String) -> Task {
        Task { id, taskname }
    }

    pub fn to_msg(&self) -> r2r::std_msgs::msg::String {
        r2r::std_msgs::msg::String {
            data: self.taskname.clone(),
        }
    }
}
