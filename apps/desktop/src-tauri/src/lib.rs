mod commands;
mod components;
mod discovery;
mod error;
mod logging;
mod models;
mod moonlight;
mod state;
mod store;
mod sunshine;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, WindowEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::state::AppState;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main(app);
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        toggle_hud(app);
                    }
                })
                .build(),
        )
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let store = store::Store::new(data_dir)?;
            logging::init(&store.logs_dir);
            logging::push(
                "info",
                "app",
                &format!("Originium Stream v{} 启动", env!("CARGO_PKG_VERSION")),
            );

            let state = AppState::new(store)?;
            let hotkey = state.snapshot().settings.global_hotkey.clone();
            app.manage(state);

            setup_tray(app)?;
            if let Ok(shortcut) = hotkey.parse::<tauri_plugin_global_shortcut::Shortcut>() {
                let _ = app.global_shortcut().register(shortcut);
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let app = window.app_handle();
                    let state = app.state::<AppState>();
                    let close_to_tray = state.config.lock().unwrap().settings.close_to_tray;
                    if close_to_tray {
                        api.prevent_close();
                        let _ = window.hide();
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::get_bootstrap,
            commands::system::get_environment,
            commands::system::get_app_logs,
            commands::system::clear_app_logs,
            commands::system::get_log_path,
            commands::system::set_autostart,
            commands::system::get_autostart,
            commands::system::set_global_hotkey,
            commands::system::show_main_window,
            commands::system::quit_app,
            commands::system::toggle_hud,
            commands::system::show_hud,
            commands::system::export_diagnostics,
            commands::system::open_main_window,
            commands::system::save_app_settings,
            commands::system::complete_onboarding,
            commands::hosts::list_hosts,
            commands::hosts::add_host,
            commands::hosts::update_host,
            commands::hosts::remove_host,
            commands::hosts::test_host,
            commands::hosts::connect_host,
            commands::hosts::discover_hosts,
            commands::hosts::add_discovered_host,
            commands::hosts::set_active_host,
            commands::hosts::reset_host_fingerprint,
            commands::sunshine::get_config,
            commands::sunshine::save_config,
            commands::sunshine::get_config_locale,
            commands::sunshine::get_apps,
            commands::sunshine::save_apps,
            commands::sunshine::delete_app,
            commands::sunshine::close_app,
            commands::sunshine::get_clients,
            commands::sunshine::unpair_client,
            commands::sunshine::unpair_all,
            commands::sunshine::update_client,
            commands::sunshine::get_pending_pairings,
            commands::sunshine::submit_pin,
            commands::sunshine::cancel_pairing,
            commands::sunshine::change_password,
            commands::sunshine::get_host_logs,
            commands::sunshine::restart_host,
            commands::sunshine::browse_host,
            commands::sunshine::get_cover,
            commands::sunshine::get_permissions,
            commands::sunshine::request_permission,
            commands::sunshine::get_virtual_input_status,
            commands::sunshine::get_virtual_input_license,
            commands::sunshine::reset_display_persistence,
            commands::sunshine::get_csrf_token,
            commands::sunshine::pair_with_pin,
            commands::components::list_components,
            commands::components::get_component_manifest,
            commands::components::install_component,
            commands::components::remove_component,
            commands::components::get_service_status,
            commands::components::start_sunshine_service,
            commands::components::stop_sunshine_service,
            commands::components::install_sunshine_service,
            commands::components::add_firewall_rules,
            commands::components::set_sunshine_credentials,
            commands::components::detect_components,
            commands::components::set_component_path,
            commands::components::validate_component_path,
            commands::moonlight::get_moonlight_settings,
            commands::moonlight::save_moonlight_settings,
            commands::moonlight::get_stream_profile,
            commands::moonlight::save_stream_profile,
            commands::moonlight::launch_session,
            commands::moonlight::stop_session,
            commands::moonlight::get_session,
            commands::moonlight::quit_remote,
            commands::theme::get_theme,
            commands::theme::list_theme_presets,
            commands::theme::save_theme,
            commands::theme::export_theme,
            commands::theme::import_theme,
            commands::theme::import_asset,
            commands::theme::list_assets,
            commands::theme::get_asset_data_url,
            commands::theme::get_raw_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出 Originium Stream", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .tooltip("Originium Stream")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

fn show_main(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

pub(crate) fn toggle_hud(app: &AppHandle) {
    if let Some(hud) = app.get_webview_window("hud") {
        let visible = hud.is_visible().unwrap_or(false);
        if visible {
            let _ = hud.hide();
        } else {
            let _ = hud.show();
            let _ = hud.set_focus();
        }
    }
}
