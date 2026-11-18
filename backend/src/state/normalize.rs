pub(super) fn normalize_pet_name(name: &mut String) {
    let trimmed = name.trim();
    let legacy_ascii = trimmed.eq_ignore_ascii_case("yuzu")
        || trimmed.eq_ignore_ascii_case("nagi")
        || trimmed.eq_ignore_ascii_case("xiaozhu");
    if trimmed == "小驻" || trimmed == "凪" || legacy_ascii || trimmed.is_empty() {
        *name = "小柚".into();
    }
}

pub(super) fn normalize_character_style(style: &mut String) {
    use crate::types::character;
    if !character::is_style(style) {
        *style = "cutegirl".into();
    }
}

pub(super) fn normalize_pet_skin(style: &str, skin: &mut String) {
    use crate::types::character;
    let legacy = match skin.as_str() {
        "female-person" => Some("coral"),
        "male-person" => Some("mint"),
        "female-adventurer" => Some("peach"),
        "male-adventurer" => Some("night"),
        _ => None,
    };
    if let Some(value) = legacy {
        *skin = value.into();
    }
    if !character::is_skin_for_style(style, skin) {
        *skin = character::default_skin(style).into();
    }
}

pub(super) fn normalize_timer_presets(presets: &mut Vec<u32>) {
    if presets.as_slice() == [5, 10, 15, 30, 60] {
        *presets = vec![5, 10, 15, 25, 30, 60];
    }
}
