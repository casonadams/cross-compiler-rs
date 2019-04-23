# cross-compiler-rs
Simple docker container for compiling rust applications using mipsel-openwrt-linux-musl compiler with openssl support and hopefully others.

## Support for openwrt build
```bash
cat /etc/openwrt_release
DISTRIB_ID='LEDE'
DISTRIB_RELEASE='17.01.4'
DISTRIB_REVISION='r3560-79f57e422d'
DISTRIB_CODENAME='reboot'
DISTRIB_TARGET='ramips/mt7620'
DISTRIB_ARCH='mipsel_24kc'
DISTRIB_DESCRIPTION='LEDE Reboot 17.01.4 3.19.1-11-gf9d1bf1'
DISTRIB_TAINTS='no-all busybox'

cat /proc/version
Linux version 4.4.92 (builder@f676f8f47e6f) (gcc version 5.4.0 (LEDE GCC 5.4.0 r3560-79f57e422d) )

cat /proc/cpuinfo
system type             : MediaTek MT7620A ver:2 eco:6
machine                 : Ralink MT7620a + MT7610e evaluation board
processor               : 0
cpu model               : MIPS 24KEc V5.0
BogoMIPS                : 385.84
wait instruction        : yes
microsecond timers      : yes
tlb_entries             : 32
extra interrupt vector  : yes
hardware watchpoint     : yes, count: 4, address/irw mask: [0x0ffc, 0x0ffc, 0x0ffb, 0x0ffb]
isa                     : mips1 mips2 mips32r1 mips32r2
ASEs implemented        : mips16 dsp
shadow register sets    : 1
kscratch registers      : 0
package                 : 0
core                    : 0
VCED exceptions         : not available
VCEI exceptions         : not available
```

## Install docker and docker-compose
[Docker Docs](https://docs.docker.com/)

## Clone Repo
```bash
git clone https://github.com/casonadams/cross-compiler-rs.git
```

## Usage
### Build base images
Buid base image
```bash
cd cross-compiler-rs
docker-compose build lede
```
### Copy source to Projects
* Repos can be cloned here
* Projects dir is a shared volume so any editor can be used to edit the code.  Then run the docker container to build when ready.

### Start container
Start docker to build rust application
```bash
docker-compose run lede
```

### Build rust application for mipsel musl
```bash
cd ~/Projects/example
cargo build --release --target=mipsel-unknown-linux-musl
mipsel-openwrt-linux-strip target/mipsel-unknown-linux-musl/release/<bin_file>
```
