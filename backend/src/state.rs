use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard};

use tauri::tray::TrayIcon;

use crate::error::AppError;
use crate::pet::PetService;
use crate::storage::Db;
use crate::timer::TimerService;
use crate::types::{PetSnapshot, ReminderRow, Settings, TimerSnapshot};

/// Map legacy skin ids to the new 16-skin catalog so existing installs
/// keep working after the character pack was replaced.
fn normalize_pet_name(name: &mut String) {
    let trimmed = name.trim();
    let legacy_ascii = trimmed.eq_ignore_ascii_case("yuzu")
        || trimmed.eq_ignore_ascii_case("nagi")
        || trimmed.eq_ignore_ascii_case("xiaozhu");
    if trimmed == "小驻" || trimmed == "凪" || legacy_ascii || trimmed.is_empty() {
        *name = "小柚".into();
    }
}

fn normalize_pet_skin(skin: &mut String) {
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

pub struct AppState {
    db: Db,
    settings: Mutex<Settings>,
    pub pet: Mutex<PetService>,
    pub timer: Mutex<TimerService>,
    pub reminders: Mutex<Vec<ReminderRow>>,
    timer_expired: AtomicBool,
    tray: Mutex<Option<TrayIcon>>,
    tray_available: AtomicBool,
}

impl AppState {
    pub fn new(db: Db) -> Result<Self, AppError> {
        let mut settings = db
            .get_setting("settings")?
            .and_then(|raw| match serde_json::from_str::<Settings>(&raw) {
                Ok(settings) => Some(settings),
                Err(error) => {
                    tracing::warn!(?error, "failed to parse stored settings; using defaults");
                    None
                }
            })
            .unwrap_or_default();
        normalize_pet_skin(&mut settings.pet_skin);
        let previous_name = settings.pet_name.clone();
        normalize_pet_name(&mut settings.pet_name);
        if settings.pet_name != previous_name {
            db.set_setting("settings", &serde_json::to_string(&settings)?)?;
        }

        let (mood, energy) = db.load_pet_state()?;
        let (timer, timer_expired) =
            TimerService::restore(db.get_timer_snapshot()?.as_deref(), &settings.pomodoro);
        let reminders = crate::reminders::load(&db, &settings.reminders)?;

        Ok(Self {
            db,
            settings: Mutex::new(settings),
            pet: Mutex::new(PetService::new(mood, energy)),
            timer: Mutex::new(timer),
            reminders: Mutex::new(reminders),
            timer_expired: AtomicBool::new(timer_expired),
            tray: Mutex::new(None),
            tray_available: AtomicBool::new(false),
        })
    }

    pub fn set_tray(&self, tray: TrayIcon) {
        if let Ok(mut guard) = self.tray.lock() {
            *guard = Some(tray);
        }
    }

    pub fn set_tray_available(&self, available: bool) {
        self.tray_available.store(available, Ordering::Relaxed);
    }

    /// Click-through needs at least one way back: tray (M0) or global shortcut (M2).
    pub fn has_recovery_entry(&self) -> bool {
        self.tray_available.load(Ordering::Relaxed)
    }

    fn lock(&self) -> Result<MutexGuard<'_, Settings>, AppError> {
        self.settings
            .lock()
            .map_err(|_| AppError::Internal("settings lock poisoned".into()))
    }

    pub fn snapshot(&self) -> Result<Settings, AppError> {
        Ok(self.lock()?.clone())
    }

    pub fn update<F>(&self, change: F) -> Result<Settings, AppError>
    where
        F: FnOnce(&mut Settings),
    {
        let mut guard = self.lock()?;
        let mut next = guard.clone();
        change(&mut next);
        self.db
            .set_setting("settings", &serde_json::to_string(&next)?)?;
        *guard = next.clone();
        Ok(next)
    }

    pub fn pet_snapshot(&self) -> Result<PetSnapshot, AppError> {
        let pet = self
            .pet
            .lock()
            .map_err(|_| AppError::Internal("pet lock poisoned".into()))?;
        Ok(pet.snapshot())
    }

    pub fn save_pet_if_dirty(&self) -> Result<(), AppError> {
        let mut pet = self
            .pet
            .lock()
            .map_err(|_| AppError::Internal("pet lock poisoned".into()))?;
        if pet.take_dirty() {
            self.db.save_pet_state(pet.mood(), pet.energy())?;
        }
        Ok(())
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    pub fn timer_snapshot(&self) -> Result<TimerSnapshot, AppError> {
        let timer = self
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        Ok(timer.snapshot())
    }

    pub fn timer_expired(&self) -> bool {
        self.timer_expired.swap(false, Ordering::Relaxed)
    }
}
