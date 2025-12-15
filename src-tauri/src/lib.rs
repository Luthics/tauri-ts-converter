use tauri::{AppHandle, Emitter};
use tauri_plugin_clipboard_manager::ClipboardExt;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Clone)]
struct ClipboardEvent {
    content: String,
}

// 异步轮询剪切板函数
fn start_clipboard_polling(app: AppHandle) {
    let last_content = Arc::new(Mutex::new(String::new()));
    let last_content_clone = Arc::clone(&last_content);

    // 使用 tokio 异步任务
    tauri::async_runtime::spawn(async move {
        loop {
            if let Ok(text) = app.clipboard().read_text() {
                let mut last = last_content_clone.lock().unwrap();
                if *last != text {
                    *last = text.clone();
                    let event = ClipboardEvent { content: text };
                    let _ = app.emit("clipboard", event);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();
            start_clipboard_polling(app_handle.clone()); // 启动剪切板轮询
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
