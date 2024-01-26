ARG BASE_IMAGE=docker.io/ubuntu:22.04

# development environment
FROM ${BASE_IMAGE} AS build-env
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        ca-certificates clang clang-format cmake curl g++-12 gcc-multilib git \
        libelf-dev libfmt-dev libgtest-dev libmsgsl-dev \
        libpcap-dev libspdlog-dev liburcu-dev llvm m4 make pkg-config \
        redis-server gcovr

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        net-tools vim iputils-ping openssh-client openssh-server tcpdump iptables gdb \
        iproute2 libsnappy-dev

WORKDIR /rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
#RUN . "/root/rust_study/.cargo/env"

