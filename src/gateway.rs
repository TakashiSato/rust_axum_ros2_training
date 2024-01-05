use r2r::std_msgs;
use r2r::QosProfile;

use crate::user::User;

pub struct Gateway {
    node: r2r::Node,
    user_pub: r2r::Publisher<std_msgs::msg::String>,
}

impl Gateway {
    pub fn new() -> Result<Gateway, Box<dyn std::error::Error>> {
        let ctx = r2r::Context::create()?;
        let mut node = r2r::Node::create(ctx, "rust_axum_ros2_training_node", "")?;

        let user_pub =
            node.create_publisher::<std_msgs::msg::String>("user", QosProfile::default())?;

        Ok(Gateway {
            node: node,
            user_pub: user_pub,
        })
    }

    pub fn publish_user(&self, user: User) {
        let msg = user.to_msg();
        self.user_pub.publish(&msg).unwrap();
    }
}
