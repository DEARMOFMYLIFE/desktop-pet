pub struct TrayLabels {
    pub toggle_pet: &'static str,
    pub start_focus: &'static str,
    pub pause_focus: &'static str,
    pub stop_timer: &'static str,
    pub bubble: &'static str,
    pub click_through: &'static str,
    pub settings: &'static str,
    pub onboarding: &'static str,
    pub quit: &'static str,
}

pub struct PetLabels {
    pub start_focus: &'static str,
    pub click_through: &'static str,
    pub settings: &'static str,
    pub hide: &'static str,
    pub quit: &'static str,
}

pub fn tray(language: &str) -> TrayLabels {
    if language == "en-US" {
        TrayLabels {
            toggle_pet: "Show / Hide Pet",
            start_focus: "Start Focus (25 min)",
            pause_focus: "Pause / Resume Focus",
            stop_timer: "Stop Timer",
            bubble: "Show Bubble",
            click_through: "Toggle Click-through",
            settings: "Settings…",
            onboarding: "Preview Onboarding",
            quit: "Quit",
        }
    } else {
        TrayLabels {
            toggle_pet: "显示 / 隐藏角色",
            start_focus: "开始专注 25 分钟",
            pause_focus: "暂停 / 继续专注",
            stop_timer: "停止计时",
            bubble: "显示气泡",
            click_through: "切换点击穿透",
            settings: "设置…",
            onboarding: "预览首次引导",
            quit: "退出应用",
        }
    }
}

pub fn pet(language: &str) -> PetLabels {
    if language == "en-US" {
        PetLabels {
            start_focus: "Start Focus (25 min)",
            click_through: "Click-through",
            settings: "Settings…",
            hide: "Hide Pet",
            quit: "Quit",
        }
    } else {
        PetLabels {
            start_focus: "开始专注 25 分钟",
            click_through: "点击穿透",
            settings: "设置…",
            hide: "隐藏角色",
            quit: "退出应用",
        }
    }
}

pub fn settings_title(language: &str, name: &str) -> String {
    if language == "en-US" {
        format!("{name} Settings")
    } else {
        format!("{name} 设置")
    }
}

pub fn onboarding_title(language: &str, name: &str) -> String {
    if language == "en-US" {
        format!("Welcome to {name}")
    } else {
        format!("欢迎来到{name}")
    }
}
