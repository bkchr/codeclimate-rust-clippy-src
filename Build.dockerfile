FROM ubuntu
MAINTAINER Bastian Köcher <codeclimate@kchr.de>

ENV DEBIAN_FRONTEND noninteractive

ENV LANG C.UTF-8

RUN \
  apt-get update && \
  apt-get -y install \
          git \
          curl \
          gcc \
          libcurl4-openssl-dev \
          libelf-dev \
          libdw-dev \
          binutils-dev \
          cmake \
          libjson-c-dev \
          libjson-c2 \
          libssl-dev \
          openssl \
          pkg-config \
          wget \
          unzip \
          python \
          libiberty-dev

ENV CARGO_HOME=/app/.cargo/
ENV RUSTUP_HOME=/app/.rustup/

ADD install-rust.sh /app/
RUN /app/install-rust.sh 

ENV PATH=/app/.cargo/bin:$PATH

WORKDIR /src