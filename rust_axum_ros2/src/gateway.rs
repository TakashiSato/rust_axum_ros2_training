use crate::error::Error;
use crate::models::task::Task;
use crate::models::user::User;
use crate::trajectory::FollowJointTrajectoryActionExecutor;
use arci_ros2::Node;
use r2r::{std_msgs, QosProfile};
use std::time::Duration;

pub struct Gateway {
    node: Node,
    user_pub: r2r::Publisher<std_msgs::msg::String>,
    task_pub: r2r::Publisher<std_msgs::msg::String>,
    follow_joint_trajectory_action_executor: FollowJointTrajectoryActionExecutor,
}

impl Gateway {
    pub fn new(name: &str, namespace: &str) -> Result<Gateway, Box<dyn std::error::Error>> {
        let node = Node::new(name, namespace)?;
        node.run_spin_thread(Duration::from_millis(100));

        let user_pub = node
            .r2r()
            .create_publisher::<std_msgs::msg::String>("user", QosProfile::default())?;

        let task_pub = node
            .r2r()
            .create_publisher::<std_msgs::msg::String>("task", QosProfile::default())?;

        let follow_joint_trajectory_action_executor =
            FollowJointTrajectoryActionExecutor::new(node.clone(), "follow_joint_trajectory");

        Ok(Gateway {
            node,
            user_pub,
            task_pub,
            follow_joint_trajectory_action_executor,
        })
    }

    pub fn publish_user(&self, user: User) -> r2r::Result<()> {
        let msg = user.to_msg();
        self.user_pub.publish(&msg)
    }

    pub fn publish_task(&self, task: Task) -> r2r::Result<()> {
        let msg = task.to_msg();
        self.task_pub.publish(&msg)
    }

    pub fn execute_follow_joint_trajectory(&self) -> r2r::Result<tokio::task::JoinHandle<()>> {
        self.follow_joint_trajectory_action_executor.send_goal()
    }

    pub fn cancel_follow_joint_trajectory(&self) -> Result<tokio::task::JoinHandle<()>, Error> {
        self.follow_joint_trajectory_action_executor.cancel_goal()
    }
}
