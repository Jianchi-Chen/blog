use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Manager};

pub fn load_system_tray(app_handle: &App) -> tauri::Result<()> {
    let show_version = MenuItem::with_id(
        app_handle,
        "show_version",
        &format!("版本: {}", env!("CARGO_PKG_VERSION")),
        false,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app_handle, "quit", "退出", true, None::<&str>)?;
    let hide = MenuItem::with_id(app_handle, "hide", "隐藏", true, None::<&str>)?;
    let show = MenuItem::with_id(app_handle, "show", "显示", true, None::<&str>)?;
    let menu = Menu::with_items(app_handle, &[&show_version, &hide, &show, &quit])?;

    TrayIconBuilder::new()
        // 把图标打进二进制，发布时不用带外部文件
        .icon(Image::from_bytes(include_bytes!("../icons/cjc.png"))?)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| handle_menu_event(app, event.id.as_ref()))
        .on_tray_icon_event(|tray, event| handle_tray_event(tray.app_handle(), event))
        .build(app_handle)?;

    Ok(())
}

fn handle_menu_event(app: &AppHandle, event_id: &str) {
    match event_id {
        "quit" => app.exit(0),
        "show" => show_main_window(app),
        "hide" => hide_main_window(app),
        _ => {}
    }
}

fn handle_tray_event(app: &AppHandle, event: TrayIconEvent) {
    if let TrayIconEvent::DoubleClick { .. } = event {
        show_main_window(app);
    }
}
fn show_main_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn hide_main_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}
