#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
const SIZE: tauri::Size = tauri::Size::Physical(tauri::PhysicalSize{width: 256, height: 256});
fn main() {
  use tauri::async_runtime as rt;
  use tauri::Window;


  tauri::Builder::default()
    .setup(move |app|{
      use tauri::Manager;
      let window = app.get_window("main").unwrap();
      window.set_min_size(Some(SIZE)).unwrap();
      window.set_size(SIZE).unwrap();
      window.center().unwrap();
      let hwnd = window.hwnd().unwrap().0;
      let _pre_val;
      let hwnd = windows::Win32::Foundation::HWND(hwnd);
      unsafe {
        use windows::Win32::UI::WindowsAndMessaging::*;
        let nindex = GWL_EXSTYLE;
        let style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST;
        _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
      };
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
