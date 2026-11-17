use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tauri::{WebviewWindow, WindowEvent};

/// Restore the persisted click-through mode only after the GTK window exists.
/// The first resize/focus event is a reliable signal that the native window is
/// realized; the atomic guard prevents later event handlers from re-enabling it.
pub(super) fn restore_click_through_after_show(window: &WebviewWindow) {
    let applied = Arc::new(AtomicBool::new(false));

    let window_for_event = window.clone();
    let applied_for_event = applied.clone();
    window.on_window_event(move |event| {
        if matches!(event, WindowEvent::Resized(_) | WindowEvent::Focused(_))
            && !applied_for_event.swap(true, Ordering::Relaxed)
        {
            if let Err(error) = window_for_event.set_ignore_cursor_events(true) {
                tracing::warn!(?error, "click-through restore failed");
            } else {
                tracing::info!("click-through restored after window event");
            }
        }
    });

    // Some window managers do not emit a resize event for a window that is
    // shown at its configured size. Try once more after the GTK loop has had
    // time to realize the window, while avoiding a call before that point.
    let window_for_task = window.clone();
    let applied_for_task = applied.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(350)).await;
        if applied_for_task.swap(true, Ordering::Relaxed) {
            return;
        }
        if !window_for_task.is_visible().unwrap_or(false) {
            return;
        }
        if let Err(error) = window_for_task.set_ignore_cursor_events(true) {
            tracing::warn!(?error, "click-through restore fallback failed");
        } else {
            tracing::info!("click-through restored after show fallback");
        }
    });
}

#[cfg(target_os = "linux")]
pub fn refresh_pet_surface(app: &tauri::AppHandle) {
    use tauri::Manager;

    let Some(pet) = app.get_webview_window(crate::windows::PET) else {
        return;
    };
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(350)).await;
        let _ = app.run_on_main_thread(move || {
            use gdkx11::X11Window;
            use gtk::glib::Cast;
            use gtk::prelude::*;
            use x11rb::connection::Connection;
            if let Ok(window) = pet.gtk_window() {
                if let Some(gdk_window) = window.window() {
                    let Ok(x11_window) = gdk_window.clone().downcast::<X11Window>() else {
                        return;
                    };
                    let xid = x11_window.xid() as u32;
                    let (x, y) = gdk_window.position();
                    let nudge = if x > 0 { x - 1 } else { x + 1 };
                    // xdotool asks the window manager to move the window, which
                    // reliably produces the compositor damage that a raw
                    // XConfigureWindow request sometimes skips.
                    let moved = std::process::Command::new("xdotool")
                        .args([
                            "windowmove",
                            &xid.to_string(),
                            &x.to_string(),
                            &y.to_string(),
                        ])
                        .status()
                        .map(|status| status.success())
                        .unwrap_or(false);
                    if !moved {
                        if let Ok((conn, _)) = x11rb::connect(None) {
                            let _ = x11rb::protocol::xproto::configure_window(
                                &conn,
                                xid,
                                &x11rb::protocol::xproto::ConfigureWindowAux::new().x(nudge),
                            );
                            let _ = x11rb::protocol::xproto::configure_window(
                                &conn,
                                xid,
                                &x11rb::protocol::xproto::ConfigureWindowAux::new().x(x).y(y),
                            );
                            let _ = conn.flush();
                        }
                    }
                }
            }
        });
    });
}
