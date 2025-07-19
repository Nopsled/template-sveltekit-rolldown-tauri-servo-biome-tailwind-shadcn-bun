#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri_runtime_verso::builder()
        .run(tauri::generate_context!())
        .unwrap();
}
