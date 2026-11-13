use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PetPosition {
    pub monitor: Option<String>,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PomodoroSettings {
    pub focus_min: u32,
    pub short_break_min: u32,
    pub long_break_min: u32,
    pub rounds: u32,
    pub auto_start_next: bool,
}

impl Default for PomodoroSettings {
    fn default() -> Self {
        Self {
            focus_min: 25,
            short_break_min: 5,
            long_break_min: 15,
            rounds: 4,
            auto_start_next: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ShortcutSettings {
    pub focus: String,
    pub click_through: String,
    pub toggle_visible: String,
}

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self {
            focus: "CmdOrCtrl+Alt+P".into(),
            click_through: "CmdOrCtrl+Alt+X".into(),
            toggle_visible: "CmdOrCtrl+Alt+H".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReminderSettings {
    pub water_enabled: bool,
    pub water_interval_min: u32,
    pub sedentary_enabled: bool,
    pub sedentary_interval_min: u32,
    pub eyes_enabled: bool,
    pub eyes_interval_min: u32,
    pub active_start: String,
    pub active_end: String,
    pub focus_behavior: String,
    pub snooze_min: u32,
}

impl Default for ReminderSettings {
    fn default() -> Self {
        Self {
            water_enabled: true,
            water_interval_min: 45,
            sedentary_enabled: true,
            sedentary_interval_min: 60,
            eyes_enabled: true,
            eyes_interval_min: 20,
            active_start: "08:00".into(),
            active_end: "22:00".into(),
            focus_behavior: "notify_only".into(),
            snooze_min: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub language: String,
    pub pet_name: String,
    pub pet_size: String,
    pub pet_skin: String,
    #[serde(default = "default_app_icon")]
    pub app_icon: String,
    pub pet_opacity: f32,
    pub click_through: bool,
    pub always_on_top: bool,
    pub reduce_motion: String,
    pub autostart: bool,
    pub first_run: bool,
    pub pet_position: Option<PetPosition>,
    pub pomodoro: PomodoroSettings,
    pub timer_presets: Vec<u32>,
    pub shortcuts: ShortcutSettings,
    pub reminders: ReminderSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "system".into(),
            pet_name: "凪".into(),
            pet_size: "M".into(),
            pet_skin: "coral".into(),
            app_icon: "clock".into(),
            pet_opacity: 1.0,
            click_through: false,
            always_on_top: true,
            reduce_motion: "system".into(),
            autostart: false,
            first_run: true,
            pet_position: None,
            pomodoro: PomodoroSettings::default(),
            timer_presets: vec![5, 10, 15, 30, 60],
            shortcuts: ShortcutSettings::default(),
            reminders: ReminderSettings::default(),
        }
    }
}

fn default_app_icon() -> String {
    "clock".into()
}
