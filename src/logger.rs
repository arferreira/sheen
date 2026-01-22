use chrono::Local;
use owo_colors::OwoColorize;
use std::fmt::Debug;

use crate::Level;

pub struct Logger {
    level: Level,
    show_timestamp: bool,
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(&self, level: Level) -> bool {
        level >= self.level
    }

    pub fn timestamp(mut self, show: bool) -> Self {
        self.show_timestamp = show;
        self
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn info(&self, message: &str, fields: &[(&str, &dyn Debug)]) {
        self.log(Level::Info, message, fields);
    }

    pub fn debug(&self, message: &str, fields: &[(&str, &dyn Debug)]) {
        self.log(Level::Debug, message, fields);
    }
    pub fn trace(&self, message: &str, fields: &[(&str, &dyn Debug)]) {
        self.log(Level::Trace, message, fields);
    }

    pub fn warn(&self, message: &str, fields: &[(&str, &dyn Debug)]) {
        self.log(Level::Warn, message, fields);
    }

    pub fn error(&self, message: &str, fields: &[(&str, &dyn Debug)]) {
        self.log(Level::Error, message, fields);
    }

    pub fn log(&self, level: Level, message: &str, fields: &[(&str, &dyn Debug)]) {
        if !self.enabled(level) {
            return;
        }

        if self.show_timestamp {
            let ts = Local::now().format("%H:%M:%S").to_string();
            eprint!("{} ", ts.dimmed())
        }

        let level_str = format!("{:<5}", level.as_str());
        let level_str = match level {
            Level::Trace => level_str.dimmed().to_string(),
            Level::Info => level_str.cyan().to_string(),
            Level::Warn => level_str.yellow().to_string(),
            Level::Debug => level_str.magenta().to_string(),
            Level::Error => level_str.red().to_string(),
        };

        eprint!("{} {}", level_str, message);
        for (key, value) in fields {
            eprint!(" {}={:?}", key, value);
        }
        eprintln!();
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            level: Level::Info,
            show_timestamp: true,
        }
    }
}
