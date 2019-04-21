FROM ubuntu:18.04

# The Rust toolchain to use when building our image
ARG TOOLCHAIN=stable
ARG TARGET=mipsel-openwrt-linux-musl
ARG RUST_TARGET=mipsel-unknown-linux-musl
ARG OPENSSL_ARCH=linux-mips32

ENV RUST_MUSL_CROSS_TARGET=$TARGET

# Make sure we have basic dev tools for building C libraries.  Our goal
# here is to support the musl-libc builds and Cargo builds needed for a
# large selection of the most popular crates.
#
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        cmake \
        curl \
        file \
        git \
	neovim \
        sudo \
        xutils-dev \
        unzip \
	wget \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Download openwrt SDK
RUN mkdir -p /home/rust/libs /home/rust/src /usr/local/musl/bin /usr/local/musl/include

RUN wget -P /tmp/ https://archive.openwrt.org/releases/17.01.4/targets/ramips/mt7620/lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64.tar.xz
WORKDIR /tmp/
RUN xz -d lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64.tar.xz
RUN tar xf lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64.tar
RUN cp -R /tmp/lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64/staging_dir/toolchain-mipsel_24kc_gcc-5.4.0_musl-1.1.16/bin/* /usr/local/musl/bin/
RUN cp -R /tmp/lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64/include/* /usr/local/musl/include/

# Set up our path with all our binary directories, including those for the
# musl-gcc toolchain and for our Rust toolchain.
ENV PATH=/root/.cargo/bin:/usr/local/musl/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV TARGET_CC=$TARGET-gcc
ENV TARGET_CXX=$TARGET-g++
ENV TARGET_C_INCLUDE_PATH=/usr/local/musl/include/

# Install our Rust toolchain and the `musl` target.  We patch the
# command-line we pass to the installer so that it won't attempt to
# interact with the user or fool around with TTYs.  We also set the default
# `--target` to musl so that our users don't need to keep overriding it
# manually.
# Chmod 755 is set for root directory to allow access execute binaries in /root/.cargo/bin (azure piplines create own user).
RUN chmod 755 /root/ && \
    curl https://sh.rustup.rs -sqSf | \
    sh -s -- -y --default-toolchain $TOOLCHAIN && \
    rustup target add $RUST_TARGET
RUN echo "[build]\ntarget = \"$RUST_TARGET\"\n\n[target.$RUST_TARGET]\nlinker = \"$-gcc\"\n" > /root/.cargo/config

# We'll build our libraries in subdirectories of /home/rust/libs.  Please
# clean up when you're done.
WORKDIR /home/rust/libs

# Build a static library version of OpenSSL using musl-libc.  This is
# needed by the popular Rust `hyper` crate.
ARG ZLIB_VERS=1.2.11
ARG OSSL_VERS=1.0.2.r
RUN export CC=$TARGET_CC && \
    export C_INCLUDE_PATH=$TARGET_C_INCLUDE_PATH && \
    echo "Building zlib" && \
    VERS=1.2.11 && \
    CHECKSUM=c3e5e9fdd5004dcb542feda5ee4f0ff0744628baf8ed2dd5d66f8ca1197cb1a1 && \
    cd /home/rust/libs && \
    curl -sqLO https://zlib.net/zlib-$ZLIB_VERS.tar.gz && \
    echo "$CHECKSUM zlib-$ZLIB_VERS.tar.gz" > checksums.txt && \
    sha256sum -c checksums.txt && \
    tar xzf zlib-$ZLIB_VERS.tar.gz && cd zlib-$ZLIB_VERS && \
    ./configure --static --archs="-fPIC" --prefix=/usr/local/musl/bin/$TARGET && \
    make && sudo make install && \
    cd .. && rm -rf zlib-$ZLIB_VERS.tar.gz zlib-$ZLIB_VERS checksums.txt

RUN export CC=$TARGET_CC && \
    export C_INCLUDE_PATH=$TARGET_C_INCLUDE_PATH && \
    echo "Building OpenSSL" && \
    CHECKSUM=ae51d08bba8a83958e894946f15303ff894d75c2b8bbd44a852b64e3fe11d0d6 && \
    curl -sqO https://www.openssl.org/source/openssl-$OSSL_VERS.tar.gz && \
    echo "$CHECKSUM openssl-$OSSL_VERS.tar.gz" > checksums.txt && \
    sha256sum -c checksums.txt && \
    tar xzf openssl-$OSSL_VERS.tar.gz && cd openssl-$OSSL_VERS && \
    ./Configure $OPENSSL_ARCH -fPIC --prefix=/usr/local/musl/bin/$TARGET && \
    make depend && \
    make && sudo make install && \
    cd .. && rm -rf openssl-$OSSL_VERS.tar.gz openssl-$OSSL_VERS checksums.txt

ENV OPENSSL_DIR=/usr/local/musl/include/ \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ \
    DEP_OPENSSL_INCLUDE=/usr/local/musl/include/ \
    OPENSSL_LIB_DIR=/usr/local/musl/lib/ \
    OPENSSL_STATIC=1

# Remove docs and more stuff not needed in this images to make them smaller
RUN rm -rf /root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/

# Expect our source code to live in /home/rust/src
WORKDIR /home/rust/src
