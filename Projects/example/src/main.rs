extern crate pkg_config;

fn main() {
    pkg_config::probe_library("openssl").unwrap();
    println!("{:?}", pkg_config::target_supported());
}
