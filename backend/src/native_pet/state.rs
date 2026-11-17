use std::time::{Duration, Instant};

use gtk::prelude::*;

use crate::error::AppError;
use crate::types::Settings;

use super::assets;
use super::ui::{default_hint, ghost_hint, size_for};
use super::NativePet;

impl NativePet {
    pub(super) fn reload_sheet(&mut self) -> Result<(), AppError> {
        if self.manifest.is_none() {
            self.manifest = Some(assets::load_manifest(&self.app, &self.settings.pet_skin)?);
        }
        let manifest = self.manifest.as_ref().expect("manifest loaded");
        let skin = self.settings.pet_skin.clone();
        let state = self.state.clone();
        self.sheet = Some(assets::load_sheet(&self.app, &skin, &state, manifest)?);
        self.started = Instant::now();
        self.last_frame = 0;
        Ok(())
    }

    pub(super) fn apply_settings(&mut self, next: &Settings) {
        let skin_changed = next.pet_skin != self.settings.pet_skin;
        let size_changed = next.pet_size != self.settings.pet_size;
        let click_changed = next.click_through != self.settings.click_through;
        let language_changed = next.language != self.settings.language;
        let name_changed = next.pet_name != self.settings.pet_name;
        let always_changed = next.always_on_top != self.settings.always_on_top;
        self.settings = next.clone();
        if size_changed {
            let size = size_for(&next.pet_size);
            self.window.resize(size, size);
            self.area.set_size_request(size, size);
            self.input_click_through = None;
        }
        if always_changed {
            self.window.set_keep_above(next.always_on_top);
        }
        if click_changed {
            self.window.set_accept_focus(!next.click_through);
            let message = ghost_hint(&next.language, next.click_through);
            self.show_hint(message, 1800);
        }
        self.apply_input_shape();
        if name_changed {
            self.window.set_title(&next.pet_name);
        }
        if language_changed && self.hint_until.is_none() {
            self.hint.set_text(default_hint(&next.language));
        }
        if skin_changed {
            self.manifest = None;
            if let Err(error) = self.reload_sheet() {
                tracing::warn!(?error, "native pet sprite reload failed");
            }
        }
        self.area.queue_draw();
    }

    fn apply_input_shape(&mut self) {
        let Some(gdk_window) = self.window.window() else {
            return;
        };
        if self.input_click_through == Some(self.settings.click_through) {
            return;
        }
        let (width, height) = self.window.size();
        let region = if self.settings.click_through {
            cairo::Region::create()
        } else {
            cairo::Region::create_rectangle(&cairo::RectangleInt::new(0, 0, width, height))
        };
        gdk_window.input_shape_combine_region(&region, 0, 0);
        gdk_window.set_pass_through(self.settings.click_through);
        self.input_click_through = Some(self.settings.click_through);
    }

    pub(super) fn set_state(&mut self, state: &str) {
        if self.state == state {
            return;
        }
        self.state = state.to_string();
        if let Err(error) = self.reload_sheet() {
            tracing::warn!(?error, state, "native pet state load failed");
        }
        self.area.queue_draw();
    }

    pub(super) fn show_hint(&mut self, message: &str, duration_ms: u64) {
        self.hint.set_text(message);
        self.hint.show();
        self.hint_until = Some(Instant::now() + Duration::from_millis(duration_ms));
    }

    pub(super) fn tick(&mut self) {
        super::geometry::capture(self);
        if self.hint_until.is_some_and(|until| Instant::now() >= until) {
            self.hint.hide();
            self.hint_until = None;
        }
        let frame = self.frame_index();
        if frame != self.last_frame {
            self.last_frame = frame;
            self.area.queue_draw();
        }
    }

    pub(super) fn frame_index(&self) -> usize {
        let Some(sheet) = &self.sheet else {
            return 0;
        };
        if self.settings.reduce_motion == "on" || sheet.frames <= 1 {
            return 0;
        }
        let elapsed = self.started.elapsed().as_secs_f64();
        let raw = (elapsed * sheet.fps) as usize;
        if sheet.loop_animation {
            raw % sheet.frames as usize
        } else {
            raw.min(sheet.frames as usize - 1)
        }
    }
}
