use ::cbindgen::{Builder, Config};
use ::chrono::offset::Local;
use ::std::{env, path::PathBuf};
fn main(){
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("失败：环境变量`CARGO_MANIFEST_DIR`未提供");
    let now = Local::now();
    println!("cargo:rustc-env=DISTRIBUTION_DATE_TIME={}", now.format("%Y年%m月%d日 %A %H时%M分"));
    #[cfg(any(feature = "nodejs", feature = "nw"))]
    link_node_nw(cargo_manifest_dir.as_str().into());
    gen_c_header(&cargo_manifest_dir[..]);
}
#[cfg(any(feature = "nodejs", feature = "nw"))]
fn link_node_nw(mut cargo_manifest_dir: PathBuf){
    // 可参考 node_bindgen::build::configure(); 的源码
    #[cfg(feature = "nodejs")]
    {
        cargo_manifest_dir.push("nodejs");
        cargo_manifest_dir.push("v16.4.0");
    }
    #[cfg(feature = "nw")]
    {
        cargo_manifest_dir.push("nw");
        cargo_manifest_dir.push("v0.54.1");
    }
    cargo_manifest_dir.push(env::var("RWA_WIN_ARCH").expect("没有环境变量 RWA_WIN_ARCH"));
    println!("cargo:rustc-link-lib=node");
    println!("cargo:rustc-link-search={}", cargo_manifest_dir.to_str().unwrap());
}
fn gen_c_header(cargo_manifest_dir: &str){
    let out_dir = env::var("OUT_DIR").expect("失败：环境变量`OUT_DIR`未提供");
    let cargo_name = env::var("CARGO_PKG_NAME").expect("失败：环境变量`CARGO_PKG_NAME`未提供");
    Builder::new().with_config(Config::from_file({
        let mut p = PathBuf::new();
        p.push(&cargo_manifest_dir[..]);
        p.push("cbindgen.toml");
        p
    }).expect("失败：解析`cbindgen.toml`配置文件"))
    .with_crate(&cargo_manifest_dir[..])
    .generate().expect("失败：生成`Cpp`头文件")
    .write_to_file({
        let mut p = PathBuf::new();
        p.push(out_dir);
        p.push(format!("../../../{}.h", cargo_name));
        p
    });
}