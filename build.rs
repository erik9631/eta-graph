use std::env;

fn main() {
    let msize_type = env::var("MSize").unwrap_or("u32".to_string());
    println!("cargo:rustc-cfg=msize_type=\"{}\"", msize_type);
}