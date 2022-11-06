# (c) Copyright 2022 Christian Saide
# SPDX-License-Identifier: GPL-3.0-or-later

FROM rust:1-bullseye as rust

# We don't want warnings for no reason
ARG DEBIAN_FRONTEND=noninteractive

# Install dev-tooling
ENV TOOL_DEPS "curl wget telnet netcat net-tools dnsutils tree vim-tiny bash-completion sudo jq"
ENV COMPILER_DEPS "upx-ucl gcc-aarch64-linux-gnu build-essential"
ENV GUI_DEPS "libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev"
RUN apt-get update \
    && apt-get upgrade -yqq \
    && apt-get install -yqq \
    ${TOOL_DEPS} ${COMPILER_DEPS} ${GUI_DEPS} \
    && rm -rf /var/lib/apt/*

# Add our rust utils and nightly toolchain.
RUN rustup update \
    && rustup component add rust-std \
    && rustup component add rust-src \
    && rustup component add rust-analyzer \
    && rustup component add rustfmt \
    && rustup component add clippy \
    && rustup toolchain install nightly -t x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu -c llvm-tools-preview \
    && cargo install cargo-llvm-cov

# Install dive
ENV DIVE_VERSION "0.10.0"
RUN wget https://github.com/wagoodman/dive/releases/download/v${DIVE_VERSION}/dive_${DIVE_VERSION}_linux_amd64.deb \
    && apt-get install -yqq ./dive_${DIVE_VERSION}_linux_amd64.deb \
    && rm -f ./dive_${DIVE_VERSION}_linux_amd64.deb

# Install grpcurl
ENV GRPCURL_VERSION "1.8.7"
RUN wget https://github.com/fullstorydev/grpcurl/releases/download/v${GRPCURL_VERSION}/grpcurl_${GRPCURL_VERSION}_linux_x86_64.tar.gz \
    && tar xzvf grpcurl_${GRPCURL_VERSION}_linux_x86_64.tar.gz \
    && mv grpcurl /usr/bin/ \
    && rm -rf grpcurl_${GRPCURL_VERSION}_linux_x86_64.tar.gz LICENSE

# Install grpc_health_probe
ENV GRPC_HEALTH_PROBE_VERSION "v0.4.14"
RUN curl -L https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 --output grpc_health_probe \
    && chmod +x grpc_health_probe \
    && mv grpc_health_probe /usr/bin

# Install docker-compose
ENV DOCKER_COMPOSE_VERSION "v2.12.2"
RUN curl -L https://github.com/docker/compose/releases/download/${DOCKER_COMPOSE_VERSION}/docker-compose-linux-x86_64 -o /usr/bin/docker-compose \
    && chmod +x /usr/bin/docker-compose

# Install docker cli
ARG docker_gid=968
ENV DOCKER_DEPS "apt-transport-https ca-certificates curl gnupg2 software-properties-common"
RUN apt-get update \
    && apt-get install -yqq \
    ${DOCKER_DEPS} \
    && curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add - \
    && add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/debian bullseye stable" \
    && apt-get update \
    && apt-get install -yqq --no-install-recommends docker-ce-cli \
    && rm -rf /var/lib/apt/* \
    && groupadd -g ${docker_gid} docker

ENV PROTOC_VERSION "21.9"
RUN wget https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip \
    && unzip -d /usr protoc-${PROTOC_VERSION}-linux-x86_64.zip \
    && rm -rf protoc-${PROTOC_VERSION}-linux-x86_64.zip

# Add appropriate user ID.
ARG uid=1000
RUN useradd -s /bin/bash -d /home/code -u ${uid} -U -G sudo,docker code \
    && mkdir -p /home/code /opt/rift \
    && chown -R code:code /home/code /usr/local/cargo /opt/rift \
    && echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Copy in a custom bashrc with util functions.
COPY --chown=code:code dist/docker/development/.bashrc /home/code/

WORKDIR /opt/rift
