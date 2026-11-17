use chrono::{Datelike, Local, TimeZone, Timelike, Utc};

pub(super) fn text_for(kind: &str, english: bool) -> (String, String) {
    match kind {
        "water" => {
            if english {
                ("Time for water".into(), "Grab a glass of water.".into())
            } else {
                ("该喝水啦".into(), "顺手接一杯吧。".into())
            }
        }
        "sedentary" => {
            if english {
                (
                    "You've been sitting a while".into(),
                    "Stand up and stretch a little.".into(),
                )
            } else {
                ("坐太久啦".into(), "起来走两步，伸个懒腰。".into())
            }
        }
        _ => {
            if english {
                (
                    "Give your eyes a break".into(),
                    "Look 6 meters away for 20 seconds.".into(),
                )
            } else {
                ("让眼睛歇一下".into(), "看看 6 米外，休息 20 秒。".into())
            }
        }
    }
}

fn parse_hhmm(value: &str) -> Option<(u32, u32)> {
    let (hour, minute) = value.split_once(':')?;
    let hour: u32 = hour.parse().ok()?;
    let minute: u32 = minute.parse().ok()?;
    (hour < 24 && minute < 60).then_some((hour, minute))
}

pub(super) fn is_active(now: chrono::DateTime<Local>, start: &str, end: &str) -> bool {
    let Some((start_h, start_m)) = parse_hhmm(start) else {
        return true;
    };
    let Some((end_h, end_m)) = parse_hhmm(end) else {
        return true;
    };
    let current = now.hour() * 60 + now.minute();
    let start = start_h * 60 + start_m;
    let end = end_h * 60 + end_m;
    if start == end {
        return true;
    }
    if start < end {
        current >= start && current < end
    } else {
        current >= start || current < end
    }
}

pub(super) fn next_active_start(
    now: chrono::DateTime<Local>,
    start: &str,
) -> chrono::DateTime<Utc> {
    let Some((hour, minute)) = parse_hhmm(start) else {
        return now.with_timezone(&Utc);
    };
    let today = Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), hour, minute, 0)
        .single()
        .unwrap_or(now);
    let next = if today > now {
        today
    } else {
        today + chrono::Duration::days(1)
    };
    (next + chrono::Duration::minutes(5)).with_timezone(&Utc)
}
