use std::env;

fn main() {
    let msize_type = env::var("MSize").unwrap_or("u64".to_string());
    println!("cargo:rustc-cfg=msize_type=\"{}\"", msize_type);
}