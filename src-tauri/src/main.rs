#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use tauri::{Manager, Window};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

//只在调用该命令的窗口触发周期事件
// #[tauri::command]
// fn init_process(window: Window) {
//   std::thread::spawn(move || {
//     loop {
//       window.emit("event-name", Payload { message: "Tauri is awesome from backend!".into() }).unwrap();
//     }
//   });
// }

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // tauri::WindowBuilder::new(
            //     app,
            //     "external-2",
            //     tauri::WindowUrl::External("https://www.bing.com/".parse().unwrap()),
            // )
            // .build()?;
            // tauri::WindowBuilder::new(app, "local-3", tauri::WindowUrl::App("index.html".into()))
            //     .build()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_docs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn open_docs(handler: tauri::AppHandle) {
    tauri::WindowBuilder::new(
        &handler,
        "external-docs",
        tauri::WindowUrl::External("https://www.bing.com/".parse().unwrap()),
    )
    .build()
    .unwrap();
}
// fn main() {
//   tauri::Builder::default()
//     .setup(|app| {
//       let main_window = app.get_window("main").unwrap();
//       main_window.listen("event-name", |event| {
//         println!("got window event-name with payload {:?}",event.payload());
//       });

//       main_window.emit("event_name", Payload { message: "Tauri is awesome!".into() }).unwrap();

//       // let _id = app.listen_global("event-name",|event|{
//       //   println!("got event-name with payload {:?}", event.payload());
//       // });
//       // // app.unlisten(id)
//       // app.emit_all("event-name",Payload { message:"Tauri is awesome!".into() }).unwrap();
//       Ok(())
//     })
//     .invoke_handler(tauri::generate_handler![init_process])
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");
// }

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

// #[tauri::command]
// fn custom_command_error_handle() -> Result<String, String> {
//     return Err("This failed".into());
// }

// #[tauri::command]
// fn open_devtools(window: tauri::Window) -> Result<String, String> {
//     return Ok(window.label().into());
// }

// fn my_move() {
//     print!("move");
// }

// #[tauri::command]
// fn my_apphandler(app_handler: tauri::AppHandle) -> Result<String, String> {
//     let app_dir = app_handler.path_resolver().app_dir();
//     use tauri::GlobalShortcutManager;
//     app_handler
//         .global_shortcut_manager()
//         .register("CTRL + U", my_move);
//     return Ok("Hello".into());
// }
