use std::time::Duration;

use gtk::glib;
use gtk::prelude::*;
use tauri::AppHandle;

use crate::error::AppError;
use crate::types::Settings;

use super::ui::size_for;
use super::{with_pet, NativePet};

const HINT_CSS: &str = r#"
.native-pet-window, .native-pet-window .background { background-color: transparent; }
.native-pet-hint {
  background-color: rgba(40, 40, 40, 0.82);
  color: #ffffff;
  border-radius: 999px;
  padding: 1px 8px;
  font-size: 10px;
}
"#;

pub(super) fn create(
    app: &AppHandle,
    settings: &Settings,
) -> Result<(gtk::Window, gtk::DrawingArea, gtk::Label), AppError> {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title(&settings.pet_name);
    window.set_default_size(size_for(&settings.pet_size), size_for(&settings.pet_size));
    window.set_decorated(false);
    window.set_resizable(false);
    window.set_skip_taskbar_hint(true);
    window.set_skip_pager_hint(true);
    window.set_keep_above(settings.always_on_top);
    window.set_accept_focus(false);
    window.set_type_hint(gtk::gdk::WindowTypeHint::Utility);
    window.set_app_paintable(true);
    window.style_context().add_class("native-pet-window");
    if let Some(screen) = gtk::gdk::Screen::default() {
        if let Some(visual) = screen.rgba_visual() {
            window.set_visual(Some(&visual));
        }
        let provider = gtk::CssProvider::new();
        let _ = provider.load_from_data(HINT_CSS.as_bytes());
        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let overlay = gtk::Overlay::new();
    let area = gtk::DrawingArea::new();
    area.set_size_request(size_for(&settings.pet_size), size_for(&settings.pet_size));
    area.set_can_focus(false);
    area.add_events(
        gtk::gdk::EventMask::BUTTON_PRESS_MASK
            | gtk::gdk::EventMask::BUTTON_RELEASE_MASK
            | gtk::gdk::EventMask::POINTER_MOTION_MASK
            | gtk::gdk::EventMask::BUTTON_MOTION_MASK,
    );
    overlay.add(&area);

    let hint = gtk::Label::new(None);
    hint.set_halign(gtk::Align::Center);
    hint.set_valign(gtk::Align::End);
    hint.set_margin_bottom(4);
    hint.set_max_width_chars(34);
    hint.set_ellipsize(gtk::pango::EllipsizeMode::End);
    hint.set_no_show_all(true);
    hint.style_context().add_class("native-pet-hint");
    overlay.add_overlay(&hint);
    overlay.set_overlay_pass_through(&hint, true);

    window.add(&overlay);
    window.connect_delete_event(|window, _| {
        window.hide();
        glib::Propagation::Stop
    });
    area.connect_draw(|area, cr| {
        with_pet(|pet| super::draw::draw(pet, area, cr));
        glib::Propagation::Proceed
    });
    super::input::attach(app, &area, &window);
    Ok((window, area, hint))
}

pub(super) fn start_tick(app: AppHandle) {
    glib::timeout_add_local(Duration::from_millis(16), move || {
        let _ = &app;
        with_pet(NativePet::tick);
        glib::ControlFlow::Continue
    });
}

pub(super) fn start_poll(app: AppHandle) {
    glib::timeout_add_local(Duration::from_millis(100), move || {
        super::poll(&app);
        glib::ControlFlow::Continue
    });
}
