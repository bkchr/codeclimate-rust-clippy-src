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

ADD install-rust.sh /root/
RUN /root/install-rust.sh 

ENV PATH=/root/.cargo/bin:$PATH


WORKDIR /src