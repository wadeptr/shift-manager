use chrono::{Datelike, Local, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailySchedule {
    /// Time to suspend (24h, local time). e.g. "23:00"
    pub suspend_at: NaiveTime,
    /// Time to wake (24h, local time). e.g. "08:00"
    pub wake_at: NaiveTime,
    /// Which weekdays to apply (0 = Sunday … 6 = Saturday). Empty = all days.
    pub weekdays: Vec<u8>,
}

impl DailySchedule {
    pub fn new(suspend_at: NaiveTime, wake_at: NaiveTime) -> Self {
        Self {
            suspend_at,
            wake_at,
            weekdays: vec![],
        }
    }

    pub fn is_active_today(&self) -> bool {
        if self.weekdays.is_empty() {
            return true;
        }
        let dow = Local::now().weekday() as u8;
        self.weekdays.contains(&dow)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SuspendTrigger {
    Schedule(DailySchedule),
    Thermal { threshold_celsius: u32 },
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleConfig {
    pub triggers: Vec<SuspendTrigger>,
    /// Minutes of warning before a scheduled suspend fires.
    pub warning_minutes: u32,
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            triggers: vec![],
            warning_minutes: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_schedule_all_days_when_weekdays_empty() {
        let s = DailySchedule::new(
            NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(s.is_active_today());
    }

    #[test]
    fn schedule_config_default_has_no_triggers() {
        let cfg = ScheduleConfig::default();
        assert!(cfg.triggers.is_empty());
        assert_eq!(cfg.warning_minutes, 5);
    }
}
