#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

#[derive(Clone,serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let _id = app.listen_global("event-name",|event|{
        println!("got event-name with payload {:?}", event.payload());
      });
      // app.unlisten(id)
      app.emit_all("event-name",Payload { message:"Tauri is awesome!".into() }).unwrap();
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet,custom_command_error_handle])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!",name)
}

#[tauri::command]
fn custom_command_error_handle() -> Result<String, String> {
  return Err("This failed".into())
}