FROM eleidan/rust:nightly-xenial

USER phantom
RUN cargo install rustfmt-nightly \
    && echo "PATH=${PATH}:${HOME}/.cargo/bin" >> ${HOME}/.bashrc


USER root
RUN apt-get update -y \
    && apt-get upgrade -y
