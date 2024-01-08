# rust_axum_ros2

## Setup

```
mkdir -p ~/ros2_ws/src
cd ~/ros2_ws/src
git clone [this repo]
```

## How to run

### 1. launch docker container

```
cd ~/ros2_ws/src/rust_axum_ros2_training
docker compose -f ./docker/docker-compose.yml up -d
```

###  2-a. without colcon

```
docker exec -it ros2 /bin/bash
cd src/rust_axum_ros2_training
cargo run
```

- If you use `cargo watch` with hot reloading, run `cargo watch -x run` instead of `cargo run`

### 2-b. with colcon

```
docker exec -it ros2 /bin/bash
cb
cs
ros2 run rust_axum_ros2 rust_axum_ros2
```

### shutdown docker container

```
cd /path/to/rust_axum_ros2_training
docker compose -f ./docker/docker-compose.yml down
```