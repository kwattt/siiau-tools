#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod classes;
mod requests;

use tauri::{
  WindowBuilder,
  WindowUrl,
};

fn main() {
  let ctx = tauri::generate_context!();

  tauri::Builder::default()
    .setup(|app| {
      let _window = WindowBuilder::new(app, "main", WindowUrl::default())
        .title("SIIAU Tools")
        .inner_size(800.0, 550.0)
        .min_inner_size(800.0, 600.0)
        .build()
        .expect("No fue posible crear la ventana!");
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      requests::get_info_cicler,
      requests::get_info_materias
    ])    
    .run(ctx)
    .expect("ocurrió un error al ejecutar la aplicación [tauri]");
}
