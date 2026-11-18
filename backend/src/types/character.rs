pub const CLASSIC_STYLES: [&str; 2] = ["cutegirl", "cuteboy"];
pub const CHARACTER_STYLES: [&str; 10] = [
    "cutegirl",
    "cuteboy",
    "lighthero",
    "webguard",
    "ironarmor",
    "titanguard",
    "starcaptain",
    "capesentinel",
    "catmouse",
    "pighero",
];
pub const CLASSIC_SKINS: [&str; 16] = [
    "honey", "cream", "mint", "peach", "night", "rose", "sky", "lavender", "jade", "coral",
    "silver", "ocean", "berry", "cocoa", "snow", "sunny",
];
pub const HERO_SKINS: [&str; 2] = ["default", "alt"];

pub fn is_style(value: &str) -> bool {
    CHARACTER_STYLES.contains(&value)
}

pub fn is_skin_for_style(style: &str, skin: &str) -> bool {
    if CLASSIC_STYLES.contains(&style) {
        CLASSIC_SKINS.contains(&skin)
    } else if is_style(style) {
        HERO_SKINS.contains(&skin)
    } else {
        false
    }
}

pub fn default_skin(style: &str) -> &'static str {
    if CLASSIC_STYLES.contains(&style) {
        "coral"
    } else {
        "default"
    }
}
