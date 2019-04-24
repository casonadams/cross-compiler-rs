FROM ubuntu:18.04

ARG GCC_VERSION="arm-linux-gnueabihf-gcc-7"

# Rust ARGS
ARG RUST_VERSION=stable
ARG TARGET="armv7-unknown-linux-gnueabihf"
ARG LINKER="arm-linux-gnueabihf-gcc-7"

# Zlib ARGS
# https://github.com/madler/zlib/releases
ARG ZLIB_VERSION="1.2.11"

# OpenSSL ARGS
# https://github.com/openssl/openssl/releases
# Note that rust-ssl needs to support the version
# https://docs.rs/openssl/0.10.20/openssl/
ARG OPENSSL_VERSION="1_0_2q"

# Install required packages
RUN apt-get update && \
  apt-get install -y \
  curl \
  gcc-7-arm-linux-gnueabihf \
  pkg-config \
  wget \
  xz-utils

# Setup directory structure
RUN mkdir -p /root/.cargo /root/Projects /usr/local/openwrt/

ENV TARGET_CC=$GCC_VERSION
ENV TARGET_C_INCLUDE_PATH=/usr

# Download Zlib
WORKDIR /tmp/
RUN wget https://github.com/madler/zlib/archive/v$ZLIB_VERSION.tar.gz
RUN tar xf v$ZLIB_VERSION.tar.gz

# Compile zlib
WORKDIR /tmp/zlib-$ZLIB_VERSION
RUN export CC=$TARGET_CC && \
    export C_INCLUDE_PATH=$TARGET_C_INCLUDE_PATH && \
    ./configure --static --archs="-fPIC" --prefix=/usr && \
    make -j 8 && \
    make install

# Download openssl
WORKDIR /tmp
RUN wget https://github.com/openssl/openssl/archive/OpenSSL_$OPENSSL_VERSION.tar.gz
RUN tar xf OpenSSL_$OPENSSL_VERSION.tar.gz

# Compile openssl
WORKDIR /tmp/openssl-OpenSSL_$OPENSSL_VERSION
RUN export CC=$TARGET_CC && \
    export C_INCLUDE_PATH=$TARGET_C_INCLUDE_PATH && \
    ./Configure linux-armv4 -march=armv7-a -fPIC --prefix=/usr --with-zlib-lib=/usr && \
    make depend && \
    make -j 8 && \
    make install

# More info on environment settings can be found
# https://docs.rs/openssl/0.10.20/openssl/
# ENV OPENSSL_DIR=$STAGING_DIR/host/ \
ENV OPENSSL_STATIC=1 \
    OPENSSL_DIR=/usr

# Install rustup with mipsel-unknown-linux-musl target
WORKDIR /tmp
RUN curl https://sh.rustup.rs -sSf > /tmp/install_rustup.sh
RUN sh /tmp/install_rustup.sh -y --default-toolchain $RUST_VERSION

# Add cross compile option to rustup
RUN /root/.cargo/bin/rustup target add $TARGET
ENV PATH="/root/.cargo/bin:$PATH"

# Setup rust to to use mipsel-unknown-linux-musl to use the openwrt mips compiler
RUN echo "[target.TARGET_TRIPLE]\nlinker = \"LINKER_PATH\"\n\n[target.$TARGET]\nlinker = \"$LINKER\"\n" > ~/.cargo/config

# Don't forget to clean up after
RUN rm -rf /tmp/*
