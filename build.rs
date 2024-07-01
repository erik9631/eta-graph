use std::env;

fn main() {
    let isize_type = env::var("ISIZE").unwrap_or("u16".to_string());
    println!("cargo:rustc-cfg=isize_type=\"{}\"", isize_type);
}

//TODO features with this look better

// fn main() {
//     if cfg!(feature = "isize_u8") {
//         println!("cargo:rustc-cfg=isize_type=\"u8\"");
//     } else if cfg!(feature = "isize_u16") {
//         println!("cargo:rustc-cfg=isize_type=\"u16\"");
//     } else if cfg!(feature = "isize_u32") {
//         println!("cargo:rustc-cfg=isize_type=\"u32\"");
//     } else if cfg!(feature = "isize_usize") {
//         println!("cargo:rustc-cfg=isize_type=\"usize\"");
//     } else {
//         // Default
//         println!("cargo:rustc-cfg=isize_type=\"usize\"");
//     }
// }