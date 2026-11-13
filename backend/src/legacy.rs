use std::fs;

/// Move pre-rename installation directories into the new Yuzu locations.
pub fn migrate_user_dirs() {
    let Some(base) = dirs::data_local_dir() else {
        return;
    };
    for (old_name, new_name) in [("xiaozhu", "yuzu"), ("com.xiaozhu.app", "com.yuzu.app")] {
        let old = base.join(old_name);
        let new = base.join(new_name);
        if !new.exists() && old.exists() {
            if let Err(error) = fs::rename(&old, &new) {
                tracing::warn!(?error, old = %old.display(), new = %new.display(), "legacy user dir migration failed");
            }
        }
    }
}
