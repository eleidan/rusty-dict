FROM eleidan/rust:nightly-xenial

RUN apt-get update -y \
    && apt-get upgrade -y \
    && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME="/home/phantom/app/.cargo"
RUN echo "PATH=${CARGO_HOME}:${PATH}" >> $HOME/.bashrc
