pub(super) fn normalize_pet_name(name: &mut String) {
    let trimmed = name.trim();
    let legacy_ascii = trimmed.eq_ignore_ascii_case("yuzu")
        || trimmed.eq_ignore_ascii_case("nagi")
        || trimmed.eq_ignore_ascii_case("xiaozhu");
    if trimmed == "小驻" || trimmed == "凪" || legacy_ascii || trimmed.is_empty() {
        *name = "小柚".into();
    }
}

pub(super) fn normalize_pet_skin(skin: &mut String) {
    let next = match skin.as_str() {
        "female-person" => "coral",
        "male-person" => "mint",
        "female-adventurer" => "peach",
        "male-adventurer" => "night",
        "honey" | "cream" | "mint" | "peach" | "night" | "rose" | "sky" | "lavender" | "jade"
        | "coral" | "silver" | "ocean" | "berry" | "cocoa" | "snow" | "sunny" => {
            return;
        }
        _ => "coral",
    };
    *skin = next.to_string();
}

pub(super) fn normalize_timer_presets(presets: &mut Vec<u32>) {
    if presets.as_slice() == [5, 10, 15, 30, 60] {
        *presets = vec![5, 10, 15, 25, 30, 60];
    }
}
