cmake_minimum_required(VERSION 3.5)
project(rust_axum_ros2)

find_package(ament_cmake REQUIRED)

if(NOT DEFINED CMAKE_SUPPRESS_DEVELOPER_WARNINGS)
     set(CMAKE_SUPPRESS_DEVELOPER_WARNINGS 1 CACHE INTERNAL "No dev warnings")
endif()

include(r2r_cargo.cmake)

r2r_cargo(std_msgs               # just to test that it works
          control_msgs
          trajectory_msgs
          rcl                    # we need the c ros2 api
          rcl_action             # as of r2r 0.1.0, we also need the action api
        #   rust_axum_ros2_msgs  # our custom message types
)

install(PROGRAMS
  ${CMAKE_SOURCE_DIR}/../target/colcon/${PROJECT_NAME}
  DESTINATION lib/${PROJECT_NAME}
)

# we need this for ros/colcon
ament_package()
