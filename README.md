# docker-rust-env
Simple docker container for compiling rust applications

## Install docker and docker-compose
[Docker Docs](https://docs.docker.com/)

## Clone Repo
```bash
git clone https://github.com/casonadams/docker-rust-env.git
```

## Usage
### Build base images
Buid base image
```bash
docker-compose build mipsel-openwrt-linux-musl
```
### Copy source to Projects
* Repos can be cloned here

### Start container
Start docker to build rust application
```bash
docker-compose run mipsel-openwrt-linux-musl
```

### Build rust application for mipsel musl
```bash
cd ~/Projects/example
cargo build --release --target=mipsel-unknown-linux-musl
```
