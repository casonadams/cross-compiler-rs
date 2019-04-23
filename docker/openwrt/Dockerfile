FROM ubuntu:18.04

# These versions are based on https://archive.openwrt.org/releases
# For openwrt this is using: ramips, mt7620
ARG OPENWRT_VERSION="17.01.4"
ARG GCC_VERSION="5.4.0_musl-1.1.16"

# Rust ARGS
ARG RUST_VERSION=stable
ARG TARGET="mipsel-unknown-linux-musl"

# This is the complier from openwrt SDK
ARG LINKER="mipsel-openwrt-linux-musl-gcc-5.4.0"

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
  pkg-config \
  wget \
  xz-utils

# Setup directory structure
RUN mkdir -p /root/.cargo /root/Projects /usr/local/openwrt/

ENV TARGET_CC=$LINKER
ENV TARGET_C_INCLUDE_PATH=/usr/local/openwrt/include/

# Download OpenWRT SDK
RUN wget -P /tmp/ https://archive.openwrt.org/releases/$OPENWRT_VERSION/targets/ramips/mt7620/lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64.tar.xz

# Extract OpenWRT SDK
WORKDIR /tmp/
RUN xz -d lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64.tar.xz
RUN tar xf lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64.tar
RUN cp -R /tmp/lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64/staging_dir/* /usr/local/openwrt/

# Add compiler to PATH
ENV STAGING_DIR="/usr/local/openwrt/"
ENV PATH="$STAGING_DIR/toolchain-mipsel_24kc_gcc-$GCC_VERSION/bin:$PATH"

# Download Zlib
RUN wget https://github.com/madler/zlib/archive/v$ZLIB_VERSION.tar.gz
RUN tar xf v$ZLIB_VERSION.tar.gz

# Compile zlib with openwrt mips compiler and install
WORKDIR /tmp/zlib-$ZLIB_VERSION
RUN export CC=$TARGET_CC && \
    export C_INCLUDE_PATH=$TARGET_C_INCLUDE_PATH && \
    ./configure --static --archs="-fPIC" --prefix=/usr/local/ && \
    make -j 8 && \
    make install

# Download openssl
WORKDIR /tmp
RUN wget https://github.com/openssl/openssl/archive/OpenSSL_$OPENSSL_VERSION.tar.gz
RUN tar xf OpenSSL_$OPENSSL_VERSION.tar.gz

# Compile openssl with openwrt mips compiler and install
WORKDIR /tmp/openssl-OpenSSL_$OPENSSL_VERSION
RUN export CC=$TARGET_CC && \
    export C_INCLUDE_PATH=$TARGET_C_INCLUDE_PATH && \
    ./Configure linux-mips32 -fPIC --prefix=/usr/local/ --with-zlib-lib=/usr/local && \
    make depend && \
    make -j 8 && \
    make install

# More info on environment settings can be found
# https://docs.rs/openssl/0.10.20/openssl/
# ENV OPENSSL_DIR=$STAGING_DIR/host/ \
ENV OPENSSL_STATIC=1 \
    OPENSSL_DIR=/usr/local/

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
