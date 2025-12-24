use log::info;
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

    TrayIconBuilder::with_id("main")
        // 把图标打进二进制，发布时不用带外部文件
        .icon(Image::from_bytes(include_bytes!("../icons/cjc.png"))?)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| handle_menu_event(app, event.id.as_ref()))
        .on_tray_icon_event(|tray, event| handle_tray_event(tray.app_handle(), event))
        .build(app_handle)?;

    Ok(())
}

/// 更新系统托盘图标
///
/// # 参数
/// * `app` - Tauri 应用句柄
/// * `icon_path` - 图标文件的完整路径，支持本地文件系统路径
///
/// # 返回值
/// * `Ok(())` - 成功更新图标
/// * `Err(String)` - 更新失败时返回错误信息
///
/// # 示例
/// ```rust
/// update_system_tray_icon(&app_handle, "path/to/new-icon.png")?;
/// ```
pub fn update_system_tray_icon(app: &AppHandle, icon_path: &str) -> Result<(), String> {
    info!("尝试更新系统托盘图标为 {}", icon_path);

    // 从文件路径加载图标
    let icon = Image::from_path(icon_path).map_err(|e| {
        log::warn!("加载图标文件 {} 失败: {}", icon_path, e);
        format!("无法加载图标文件 {}: {}", icon_path, e)
    })?;

    // 获取托盘图标实例（获取默认托盘，如果有多个托盘可以使用 tray_by_id）
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| "未找到系统托盘实例".to_string())?;

    // 更新托盘图标
    tray.set_icon(Some(icon))
        .map_err(|e| format!("设置托盘图标失败: {}", e))?;

    info!("系统托盘图标已更新为 {}", icon_path);

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
