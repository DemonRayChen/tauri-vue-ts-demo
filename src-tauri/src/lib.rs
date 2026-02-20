use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Windows 特定的链接器指令
#[cfg(target_os = "windows")]
#[link(name = "testlib", kind = "static")]
extern "C" {
  fn add_numbers(a: i32, b: i32) -> i32;
  fn process_string(input: *const c_char) -> *mut c_char;
  fn free_string(ptr: *mut c_char);
}

// 非 Windows 平台的链接器指令
#[cfg(not(target_os = "windows"))]
#[link(name = "testlib")]
extern "C" {
  fn add_numbers(a: i32, b: i32) -> i32;
  fn process_string(input: *const c_char) -> *mut c_char;
  fn free_string(ptr: *mut c_char);
}

#[tauri::command]
fn cpp_add(a: i32, b: i32) -> i32 {
  unsafe { add_numbers(a, b) }
}

#[tauri::command]
fn cpp_process_string(input: String) -> Result<String, String> {
  unsafe {
    let c_input = CString::new(input).map_err(|e| e.to_string())?;
    let c_output = process_string(c_input.as_ptr());
    if c_output.is_null() {
        return Err("C++ function returned null".to_string());
    }

    let result = CStr::from_ptr(c_output)
    .to_string_lossy()
    .into_owned();

    free_string(c_output);

    Ok(result)
  }
}

#[tauri::command]
fn test(input: String) -> Result<String, String> {
  Ok(format!("Received from Vue: {}", input))
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      cpp_add,
      cpp_process_string,
      test
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
