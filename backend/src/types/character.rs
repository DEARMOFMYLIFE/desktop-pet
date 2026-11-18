pub const CLASSIC_STYLES: [&str; 2] = ["cutegirl", "cuteboy"];
pub const CHARACTER_STYLES: [&str; 8] = [
    "cutegirl",
    "cuteboy",
    "lighthero",
    "webguard",
    "ironarmor",
    "titanguard",
    "starcaptain",
    "capesentinel",
];
pub const CLASSIC_SKINS: [&str; 16] = [
    "honey", "cream", "mint", "peach", "night", "rose", "sky", "lavender", "jade", "coral",
    "silver", "ocean", "berry", "cocoa", "snow", "sunny",
];
pub const HERO_PALETTES: [&str; 8] = [
    "coral", "honey", "mint", "sky", "lavender", "rose", "night", "silver",
];

pub fn is_style(value: &str) -> bool {
    CHARACTER_STYLES.contains(&value)
}

pub fn characters_for_style(style: &str) -> &'static [&'static str] {
    match style {
        "lighthero" => &["girl", "boy"],
        "webguard" => &["boy", "cat"],
        "ironarmor" => &["robot", "boy"],
        "titanguard" => &["robot", "cat"],
        "starcaptain" => &["girl", "robot"],
        "capesentinel" => &["girl", "cat"],
        _ => &[],
    }
}

pub fn default_skin(style: &str) -> &'static str {
    match style {
        "lighthero" => "girl-coral",
        "webguard" => "boy-coral",
        "ironarmor" => "robot-coral",
        "titanguard" => "robot-coral",
        "starcaptain" => "girl-coral",
        "capesentinel" => "girl-coral",
        _ => "coral",
    }
}

pub fn is_skin_for_style(style: &str, skin: &str) -> bool {
    if CLASSIC_STYLES.contains(&style) {
        return CLASSIC_SKINS.contains(&skin);
    }
    let Some((character, palette)) = skin.split_once('-') else {
        return false;
    };
    characters_for_style(style).contains(&character) && HERO_PALETTES.contains(&palette)
}
