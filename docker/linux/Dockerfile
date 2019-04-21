FROM ubuntu:18.04

# These versions are based on https://archive.openwrt.org/releases
# For openwrt this is using: ramips, mt7260
ARG OPENWRT_VERSION="17.01.4"
ARG GCC_VERSION="5.4.0_musl-1.1.16"
ARG RUST_VERSION=stable

# Install required packages
RUN apt-get update && \
  apt-get install -y \
  curl \
  git \
  pkg-config \
  wget \
  xz-utils

# Setup directory structure
RUN mkdir -p /root/.cargo /root/Projects /usr/local/musl/

# Download OpenWRT SDK
RUN wget -P /tmp/ https://archive.openwrt.org/releases/$OPENWRT_VERSION/targets/ramips/mt7620/lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64.tar.xz

# Extract OpenWRT SDK
WORKDIR /tmp/
RUN xz -d lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64.tar.xz
RUN tar xf lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64.tar

# Add compiler to PATH
ENV STAGING_DIR="/tmp/lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64/staging_dir"
ENV PATH="/tmp/lede-sdk-$OPENWRT_VERSION-ramips-mt7620_gcc-$GCC_VERSION.Linux-x86_64/staging_dir/toolchain-mipsel_24kc_gcc-$GCC_VERSION/bin:$PATH"

# Clone zlib
RUN git clone --depth 1 https://github.com/madler/zlib.git /tmp/zlib

# Compile zlib with openwrt mips compiler and install
WORKDIR /tmp/zlib
RUN CHOST=mipsel-openwrt-linux-musl- ./configure --static --archs="-fPIC" --prefix=/usr/local/musl \
  && make \
  && make install

# Clone openssl
RUN git clone --depth 1 https://github.com/openssl/openssl.git /tmp/openssl

# Compile openssl with openwrt mips compiler and install
WORKDIR /tmp/openssl
RUN ./Configure linux-mips32 --cross-compile-prefix=mipsel-openwrt-linux-musl- -fPIC --with-zlib-lib=/usr/local/include --prefix=/usr/local/musl \
  && make -j 8 \
  && make install

ENV OPENSSL_DIR=/usr/local/musl/ \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include \
    DEP_OPENSSL_INCLUDE=/usr/local/musl/include \
    OPENSSL_LIB_DIR=/usr/local/musl/lib \
    OPENSSL_STATIC=1

# Install rustup with mipsel-unknown-linux-musl target
RUN curl https://sh.rustup.rs -sSf > /tmp/install_rustup.sh
RUN sh /tmp/install_rustup.sh -y --default-toolchain $RUST_VERSION
RUN /root/.cargo/bin/rustup target add mipsel-unknown-linux-musl

ENV PATH="/root/.cargo/bin:$PATH"

# Setup rust to to use mipsel-unknown-linux-musl to use the openwrt mips compiler
RUN echo '[target.TARGET_TRIPLE]\nlinker = "LINKER_PATH"\n\n[target.mipsel-unknown-linux-musl]\nlinker = "mipsel-openwrt-linux-gcc"\n' > ~/.cargo/config
