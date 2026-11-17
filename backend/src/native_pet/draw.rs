use gtk::prelude::*;

use crate::types::Settings;

use super::NativePet;

pub(super) fn draw(pet: &NativePet, area: &gtk::DrawingArea, cr: &cairo::Context) {
    let width = area.allocated_width() as f64;
    let height = area.allocated_height() as f64;
    cr.set_operator(cairo::Operator::Source);
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
    let _ = cr.paint();
    cr.set_operator(cairo::Operator::Over);

    if let Some(sheet) = &pet.sheet {
        let fw = sheet.frame_width as f64;
        let fh = sheet.frame_height as f64;
        let frame = pet.frame_index();
        let scale = width / fw;
        let _ = cr.save();
        cr.scale(scale, scale);
        cr.rectangle(0.0, 0.0, fw, fh);
        cr.clip();
        let x = -((frame as f64) * fw);
        if cr.set_source_surface(&sheet.surface, x, 0.0).is_ok() {
            let _ = cr.paint_with_alpha(sprite_opacity(&pet.settings));
        }
        let _ = cr.restore();
    }

    if pet.settings.click_through {
        draw_badge(cr, width, height);
    }
}

fn sprite_opacity(settings: &Settings) -> f64 {
    let value = settings.pet_opacity as f64;
    if settings.click_through {
        value.min(0.6)
    } else {
        value
    }
}

fn draw_badge(cr: &cairo::Context, width: f64, height: f64) {
    let cx = width - 14.0;
    let cy = height - 14.0;
    cr.set_source_rgba(0.08, 0.08, 0.08, 0.62);
    cr.arc(cx, cy, 9.0, 0.0, std::f64::consts::TAU);
    let _ = cr.fill();
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.9);
    cr.set_line_width(1.4);
    cr.rectangle(cx - 3.0, cy - 1.0, 6.0, 5.0);
    let _ = cr.stroke();
    cr.arc(cx, cy - 2.5, 2.2, std::f64::consts::PI, 0.0);
    let _ = cr.stroke();
}
