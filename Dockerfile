FROM rust:1.74 AS build-env
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        ca-certificates clang clang-format cmake curl g++-12 gcc-multilib git \
        libelf-dev libsnappy-dev libsctp-dev
