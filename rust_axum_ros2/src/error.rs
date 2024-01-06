use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("rust_axum_ros2: No joint_state is available")]
    NoValidGoalExists,
    #[error("rust_axum_ros2: Other: {:?}", .0)]
    Other(#[from] anyhow::Error),
}
