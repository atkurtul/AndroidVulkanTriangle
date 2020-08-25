
extern crate cc;
use std::env;

fn main() {
    let target = env::vars().find_map(|(k, v)| if k == "TARGET" { Some(v) } else { None }).unwrap();
    let toolchain = env::vars().find_map(|(k, v)| if k == "ANDROID_TOOLCHAIN" { Some(v) } else { None }).unwrap();
    let compiler = toolchain.clone() + "/bin/" + target.as_str() + "30-clang";
    let ar =  toolchain + "/bin/" + target.as_str() + "-ar";
    cc::Build::new()
        .file("src/vk.c")
        .compiler(compiler)
        .archiver(ar)
        .compile("libvk.a");
}