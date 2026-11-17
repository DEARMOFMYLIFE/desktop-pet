use gtk::prelude::*;

use crate::types::Settings;

use super::NativePet;

pub(super) fn draw(pet: &NativePet, area: &gtk::DrawingArea, cr: &cairo::Context) {
    let width = area.allocated_width() as f64;
    cr.set_operator(cairo::Operator::Source);
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
    let _ = cr.paint();
    cr.set_operator(cairo::Operator::Over);

    if let Some(sheet) = &pet.sheet {
        let frame_width = sheet.frame_width as f64;
        let frame_height = sheet.frame_height as f64;
        let frame = pet.frame_index();
        let scale = width / frame_width;
        let _ = cr.save();
        cr.scale(scale, scale);
        cr.rectangle(0.0, 0.0, frame_width, frame_height);
        cr.clip();
        let x = -((frame as f64) * frame_width);
        if cr.set_source_surface(&sheet.surface, x, 0.0).is_ok() {
            let _ = cr.paint_with_alpha(sprite_opacity(&pet.settings));
        }
        let _ = cr.restore();
    }

    if pet.settings.click_through {
        draw_lock_badge(cr, width);
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

fn draw_lock_badge(cr: &cairo::Context, pet_size: f64) {
    let cx = pet_size - 13.0;
    let cy = pet_size - 13.0;
    let radius = 10.0;
    let _ = cr.save();

    cr.set_source_rgba(0.20, 0.08, 0.12, 0.18);
    cr.arc(cx, cy + 1.4, radius, 0.0, std::f64::consts::TAU);
    let _ = cr.fill();

    let gradient = cairo::LinearGradient::new(cx - radius, cy - radius, cx + radius, cy + radius);
    gradient.add_color_stop_rgba(0.0, 1.0, 0.72, 0.81, 1.0);
    gradient.add_color_stop_rgba(1.0, 0.96, 0.42, 0.59, 1.0);
    let _ = cr.set_source(&gradient);
    cr.arc(cx, cy, radius, 0.0, std::f64::consts::TAU);
    let _ = cr.fill();

    cr.set_source_rgba(1.0, 1.0, 1.0, 0.95);
    cr.set_line_width(1.4);
    cr.arc(cx, cy, radius - 0.7, 0.0, std::f64::consts::TAU);
    let _ = cr.stroke();

    cr.set_source_rgba(1.0, 1.0, 1.0, 0.45);
    cr.set_line_width(1.1);
    cr.arc(
        cx,
        cy,
        radius - 3.2,
        std::f64::consts::PI * 1.05,
        std::f64::consts::PI * 1.45,
    );
    let _ = cr.stroke();

    cr.set_source_rgba(1.0, 1.0, 1.0, 0.96);
    cr.set_line_width(1.7);
    cr.set_line_cap(cairo::LineCap::Round);
    cr.arc(
        cx,
        cy - 1.6,
        2.7,
        std::f64::consts::PI,
        std::f64::consts::TAU,
    );
    let _ = cr.stroke();

    rounded_rect(cr, cx - 3.8, cy - 0.4, 7.6, 6.0, 1.8);
    let _ = cr.fill();

    cr.set_source_rgba(0.96, 0.42, 0.59, 1.0);
    draw_heart(cr, cx, cy + 2.1, 1.35);
    let _ = cr.fill();
    let _ = cr.restore();
}

fn rounded_rect(cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64, radius: f64) {
    cr.new_sub_path();
    cr.arc(
        x + width - radius,
        y + radius,
        radius,
        -std::f64::consts::FRAC_PI_2,
        0.0,
    );
    cr.arc(
        x + width - radius,
        y + height - radius,
        radius,
        0.0,
        std::f64::consts::FRAC_PI_2,
    );
    cr.arc(
        x + radius,
        y + height - radius,
        radius,
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
    );
    cr.arc(
        x + radius,
        y + radius,
        radius,
        std::f64::consts::PI,
        std::f64::consts::PI * 1.5,
    );
    cr.close_path();
}

fn draw_heart(cr: &cairo::Context, cx: f64, cy: f64, size: f64) {
    cr.move_to(cx, cy + size * 0.9);
    cr.curve_to(
        cx - size * 1.5,
        cy - size * 0.1,
        cx - size * 0.6,
        cy - size * 1.2,
        cx,
        cy - size * 0.35,
    );
    cr.curve_to(
        cx + size * 0.6,
        cy - size * 1.2,
        cx + size * 1.5,
        cy - size * 0.1,
        cx,
        cy + size * 0.9,
    );
    cr.close_path();
}
