use crate::error::Error;
use arci;
use arci_ros2::Node;
use futures::stream::StreamExt;
use r2r::{
    builtin_interfaces::msg::Time, control_msgs::action::FollowJointTrajectory,
    std_msgs::msg::Header, trajectory_msgs::msg::JointTrajectory,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::sync::broadcast;
use tokio::time::timeout;

type FollowJointTrajectoryAction = FollowJointTrajectory::Action;
type FollowJointTrajectoryActionClient = r2r::ActionClient<FollowJointTrajectoryAction>;
type FollowJointTrajectoryActionGoal = r2r::ActionClientGoal<FollowJointTrajectoryAction>;
type FollowJointTrajectoryActionGoalOption = Option<FollowJointTrajectoryActionGoal>;

pub struct FollowJointTrajectoryActionExecutor {
    action_client: FollowJointTrajectoryActionClient,
    node: Node,
    current_goal: Arc<Mutex<FollowJointTrajectoryActionGoalOption>>,
}

impl FollowJointTrajectoryActionExecutor {
    #[track_caller]
    pub fn new(node: Node, action_name: &str) -> Self {
        let action_client = node
            .r2r()
            .create_action_client::<FollowJointTrajectoryAction>(action_name)
            .unwrap();
        Self {
            action_client,
            node,
            current_goal: Arc::new(Mutex::new(None)),
        }
    }

    pub fn send_goal(&self) -> r2r::Result<tokio::task::JoinHandle<()>> {
        let node = self.node.clone();
        let current_goal = self.current_goal.clone();
        let action_client = self.action_client.clone();
        let is_available = node.r2r().is_available(&action_client)?;

        let action_handler = tokio::spawn(async move {
            // wait for action server to be available
            println!("waiting for action server...");
            if let Err(e) = timeout(Duration::from_millis(3000), is_available).await {
                println!("action server is not available: {:?}", e);
                return;
            }
            println!("action server is available");

            let now = std::time::Instant::now();
            let last_update_time = Arc::new(Mutex::new(now));
            let last_update_time_clone = last_update_time.clone();

            let is_done = Arc::new(AtomicBool::new(false));
            let is_done_clone = is_done.clone();
            let current_goal_clone = current_goal.clone();

            let (cancel_tx, mut cancel_rx1) = broadcast::channel(1);
            let mut cancel_rx2 = cancel_tx.subscribe();

            // spawn a task to handle goal request
            tokio::spawn(async move {
                let mut clock = r2r::Clock::create(r2r::ClockType::RosTime)
                    .expect("failed to create RosTime clock");
                let now = clock
                    .get_now()
                    .expect("failed to get now from RosTime clock");

                let goal = FollowJointTrajectory::Goal {
                    trajectory: JointTrajectory {
                        header: Header {
                            frame_id: "".to_string(),
                            stamp: Time {
                                sec: now.as_secs() as i32,
                                nanosec: now.subsec_nanos(),
                            },
                        },
                        joint_names: vec!["joint1".to_string(), "joint2".to_string()],
                        points: vec![],
                    },
                    ..Default::default()
                };

                let send_goal_request = action_client
                    .send_goal_request(goal)
                    .expect("failed to send goal request");

                let (goal, result, feedback) = send_goal_request
                    .await
                    .expect("goal rejected by action server");

                // update current_goal
                current_goal_clone.lock().unwrap().replace(goal.clone());

                println!("goal_accepted: {}", goal.uuid);

                // spawn a task to handle feedback
                tokio::spawn(async move {
                    let goal = goal.clone();

                    // wait for feedback
                    tokio::select! {
                        _ = feedback
                            .for_each(|msg| {
                                // update last_update_time_nsec
                                let now = std::time::Instant::now();
                                *last_update_time.lock().unwrap() = now;

                                println!(
                                    "feedback: {:?} -- {:?}",
                                    msg.header.stamp,
                                    goal.get_status()
                                );
                                std::future::ready(())
                            }) => {
                                println!("feedback finished");
                            }
                        v = cancel_rx1.recv() => {
                            match v {
                                Ok(_) => {
                                    println!("feedback cancel_rx.recv() finished");
                                }
                                Err(e) => {
                                    println!("feedback cancel_rx.recv() error: {:?}", e);
                                }
                            }
                        }
                    }
                });

                // wait for result
                tokio::select! {
                    r = result => {
                        match r {
                            Ok((status, msg)) => {
                                println!("Got result {} with msg {:?}", status, msg);
                            }
                            Err(e) => {
                                println!("Action failed: {:?}", e);
                            }
                        }
                        is_done.store(true, Ordering::Relaxed);
                    }
                    _ = cancel_rx2.recv() => {
                        println!("wait result cancel_rx.changed() finished");
                    }
                }
            });

            // check if action is completed or timed out
            let timeout = Duration::new(10, 0);
            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;

                // check if action is completed
                if is_done_clone.load(Ordering::Relaxed) {
                    println!("action completed");
                    break;
                }

                // check if action is timed out
                let last_update_time = *last_update_time_clone.lock().unwrap();
                let now = std::time::Instant::now();
                let elapsed_from_last_update = now - last_update_time;

                if elapsed_from_last_update >= timeout {
                    println!("action timed out");
                    cancel_tx.send("cancel").unwrap();
                    break;
                }
            }

            // clear current_goal
            current_goal.lock().unwrap().take();
        });

        Ok(action_handler)
    }

    pub fn cancel_goal(&self) -> Result<tokio::task::JoinHandle<()>, Error> {
        if let Some(current_goal) = self.current_goal.lock().unwrap().take() {
            println!("cancel goal: {:?}", current_goal.uuid);
            let fut = current_goal.cancel().map_err(|e| Error::Other(e.into()))?;
            let cancel_handler = tokio::spawn(async move {
                let _ = fut.await;
                println!("canceled goal: {:?}", current_goal.uuid);
            });

            return Ok(cancel_handler);
        }

        Err(Error::NoValidGoalExists)
    }
}
