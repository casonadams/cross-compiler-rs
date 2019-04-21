FROM messense/rust-musl-cross:mipsel-musl

# install required packages
RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y \
  build-essential \
  wget \
  neovim \
  curl \
  gcc \
  libc6-dev \
  xz-utils \
  libssl-dev \
  pkg-config \
  git

RUN mkdir /home/bin
RUN mkdir -p /home/rust/src

RUN wget -P /tmp/ https://archive.openwrt.org/releases/17.01.4/targets/ramips/mt7620/lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64.tar.xz

WORKDIR /tmp/
RUN xz -d lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64.tar.xz
RUN tar xf lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64.tar

RUN echo '[target.TARGET_TRIPLE]\nlinker = "LINKER_PATH"\n\n[target.mipsel-unknown-linux-musl]\nlinker = "mipsel-openwrt-linux-gcc"\n' > ~/.cargo/config

ENV STAGING_DIR="/tmp/lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64/staging_dir"

ENV PATH="/tmp/lede-sdk-17.01.4-ramips-mt7620_gcc-5.4.0_musl-1.1.16.Linux-x86_64/staging_dir/toolchain-mipsel_24kc_gcc-5.4.0_musl-1.1.16/bin:$PATH"

WORKDIR /home/rust/src
RUN git clone --depth 1 https://github.com/openssl/openssl.git

WORKDIR /home/rust/src/openssl
RUN ./Configure linux-mips32 --cross-compile-prefix=mipsel-openwrt-linux-musl- \
  && make \
  && make install
