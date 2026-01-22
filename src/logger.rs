use owo_colors::OwoColorize;

use crate::Level;

pub struct Logger {
    level: Level,
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(&self, level: Level) -> bool {
        level >= self.level
    }

    pub fn info(&self, message: &str) {
        self.log(Level::Info, message);
    }

    pub fn debug(&self, message: &str) {
        self.log(Level::Debug, message);
    }
    pub fn trace(&self, message: &str) {
        self.log(Level::Trace, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(Level::Warn, message);
    }

    pub fn error(&self, message: &str) {
        self.log(Level::Error, message);
    }

    pub fn log(&self, level: Level, message: &str) {
        if !self.enabled(level) {
            return;
        }

        let level_str = match level {
            Level::Trace => level.as_str().dimmed().to_string(),
            Level::Info => level.as_str().cyan().to_string(),
            Level::Warn => level.as_str().yellow().to_string(),
            Level::Debug => level.as_str().magenta().to_string(),
            Level::Error => level.as_str().red().to_string(),
        };

        eprintln!("{} {}", level_str, message)
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self { level: Level::Info }
    }
}
