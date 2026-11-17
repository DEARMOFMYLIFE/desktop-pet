use gtk::prelude::*;
use tauri::{AppHandle, Manager};

use crate::menu_text;
use crate::state::AppState;

pub(super) fn show(app: &AppHandle) {
    let language = app
        .state::<AppState>()
        .snapshot()
        .map(|settings| settings.language)
        .unwrap_or_else(|_| "zh-CN".into());
    let labels = menu_text::pet(&language);
    let menu = gtk::Menu::new();
    add_item(&menu, labels.start_focus, app, |app| {
        let _ = crate::timer::stop(app);
        let _ = crate::timer::start(app, crate::timer::TimerStartRequest::Pomodoro);
        let _ = crate::windows::open_bubble(app);
    });
    add_item(&menu, labels.click_through, app, |app| {
        let _ = crate::windows::toggle_click_through(app);
    });
    add_item(&menu, labels.settings, app, |app| {
        let _ = crate::windows::open_settings(app);
    });
    add_item(&menu, labels.hide, app, |app| {
        crate::windows::hide_pet(app);
    });
    add_item(&menu, labels.quit, app, |app| app.exit(0));
    menu.show_all();
    menu.popup_easy(3, gtk::current_event_time());
}

fn add_item(menu: &gtk::Menu, label: &str, app: &AppHandle, action: impl Fn(&AppHandle) + 'static) {
    let item = gtk::MenuItem::with_label(label);
    let app = app.clone();
    item.connect_activate(move |_| action(&app));
    menu.append(&item);
}
