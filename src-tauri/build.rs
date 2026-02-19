use std::env;
use std::path::PathBuf;

fn main() {
  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();


  let lib_dir = match target_os.as_str() {
    "windows" => PathBuf::from(&manifest_dir).join("libs").join("windows"),
    "macos" => PathBuf::from(&manifest_dir).join("libs").join("macos"),
    "linux" => PathBuf::from(&manifest_dir).join("libs").join("linux"),
    _ => panic!("Unsupported target OS: {}", target_os),
  };

  println!("Using library directory: {}", lib_dir.display());

  if !lib_dir.exists() {
    panic!("Library directory does not exist: {}", lib_dir.display());
  }

  // 1. 编译时链接配置
  println!("cargo:rustc-link-search=native={}", lib_dir.display());

  // 2. 设置运行时搜索路径
  println!("cargo:rustc-link-lib=dylib=testlib");

  println!("cargo:rerun-if-changed=libs/");

  tauri_build::build()
}
