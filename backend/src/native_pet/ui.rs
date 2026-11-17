pub(super) const HINT_SPACE: i32 = 30;

pub(super) fn size_for(size: &str) -> i32 {
    match size {
        "S" => 160,
        "L" => 256,
        _ => 208,
    }
}

pub(super) fn window_height(size: &str) -> i32 {
    size_for(size) + HINT_SPACE
}

pub(super) fn default_hint(language: &str) -> &'static str {
    if crate::language::is_english(language) {
        "Double-click for settings · right-click for more"
    } else {
        "双击打开设置 · 右键更多"
    }
}

pub(super) fn click_hint(language: &str) -> &'static str {
    if crate::language::is_english(language) {
        "Hehe, that tickles."
    } else {
        "嘿嘿，好痒。"
    }
}

pub(super) fn ghost_hint(language: &str, enabled: bool) -> &'static str {
    match (crate::language::is_english(language), enabled) {
        (true, true) => "Click-through on",
        (true, false) => "Click-through off",
        (false, true) => "点击穿透已开启",
        (false, false) => "点击穿透已关闭",
    }
}
