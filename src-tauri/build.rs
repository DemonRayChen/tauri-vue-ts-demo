fn main() {
  println!("cargo:rerun-if-changed=native/include/testlib.h");
  println!("cargo:rerun-if-changed=native/testlib.cpp");

  cc::Build::new()
    .cpp(true)
    .file("native/testlib.cpp")
    .include("native/include")
    .flag_if_supported("-std=c++17")
    .flag_if_supported("/std:c++17")
    .compile("testlib");

  tauri_build::build()
}
