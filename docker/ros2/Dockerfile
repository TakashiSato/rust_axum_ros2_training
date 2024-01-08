FROM osrf/ros:humble-desktop

ENV DEBCONF_NOWARNINGS=yes
ENV TZ=Asia/Tokyo

COPY ./ros2/requirements.txt /app/requirements.txt

# install dependencies
RUN apt-get update -q \
    && apt-get upgrade -yqq \
    && apt-get install -yqq \
    bash-completion \
    vim \
    tmux \
    git \
    net-tools \
    iputils-ping \
    psmisc \
    cmake \
    curl \
    wget \
    locales \
    clang \
    ros-humble-control-msgs \
    python3-rosdep \
    python3-vcstool \
    python3-colcon-common-extensions \
    python3-pip \
    && pip3 install --no-cache-dir --upgrade -r /app/requirements.txt

# clean apt cache
RUN apt-get autoremove -yqq \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# remove apt.conf.d/docker-clean to enable auto-completion of apt command
RUN rm /etc/apt/apt.conf.d/docker-clean

# install git-secrets
RUN git clone https://github.com/awslabs/git-secrets /usr/local/git-secrets \
    && cd /usr/local/git-secrets \
    && make install

# install mold
RUN git clone -b v2.4.0 https://github.com/rui314/mold.git /usr/local/mold \
    && mkdir /usr/local/mold/build \
    && cd /usr/local/mold/build \
    && ../install-build-deps.sh \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. \
    && cmake --build . -j $(nproc) \
    && cmake --build . --target install

RUN locale-gen en_US.UTF-8
ENV LANG=en_US.UTF-8 LANGUAGE=en_US:en LC_ALL=en_US.UTF-8

# create user and change to user
RUN useradd --create-home --home-dir /home/ubuntu --shell /bin/bash --user-group --groups adm,sudo ubuntu && \
    echo ubuntu:ubuntu | chpasswd && \
    echo "ubuntu ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
USER ubuntu
ENV HOME=/home/ubuntu

RUN rosdep update

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --no-modify-path
RUN echo "export PATH=\$PATH:\$HOME/.cargo/bin" >> $HOME/.bashrc

# install rust crates
ENV PATH $PATH:$HOME/.cargo/bin
RUN cargo install cargo-watch

# install cargo-ament
# RUN git clone https://github.com/tier4/cargo-ament-build.git ${RUST_HOME}/cargo-ament-build \
#     && cd ${RUST_HOME}/cargo-ament-build \
#     && cargo install --path .

# bash completion and ros settings
COPY ./ros2/.rosrc $HOME/.rosrc
RUN echo "source ~/.rosrc" >> $HOME/.bashrc

WORKDIR $HOME
ENTRYPOINT ["/ros_entrypoint.sh"]
