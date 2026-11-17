use gtk::prelude::*;
use tauri::AppHandle;

pub(super) fn show(app: &AppHandle) {
    let menu = gtk::Menu::new();
    add_item(&menu, "开始专注 25 分钟", app, |app| {
        let _ = crate::timer::stop(app);
        let _ = crate::timer::start(app, crate::timer::TimerStartRequest::Pomodoro);
        let _ = crate::windows::open_bubble(app);
    });
    add_item(&menu, "点击穿透", app, |app| {
        let _ = crate::windows::toggle_click_through(app);
    });
    add_item(&menu, "设置…", app, |app| {
        let _ = crate::windows::open_settings(app);
    });
    add_item(&menu, "隐藏角色", app, |app| {
        crate::windows::hide_pet(app);
    });
    add_item(&menu, "退出应用", app, |app| app.exit(0));
    menu.show_all();
    menu.popup_easy(3, gtk::current_event_time());
}

fn add_item(menu: &gtk::Menu, label: &str, app: &AppHandle, action: impl Fn(&AppHandle) + 'static) {
    let item = gtk::MenuItem::with_label(label);
    let app = app.clone();
    item.connect_activate(move |_| action(&app));
    menu.append(&item);
}
