use chrono::Local;
use owo_colors::OwoColorize;
use std::{fmt::Debug, io::IsTerminal};

use crate::Level;

pub struct Logger {
    level: Level,
    show_timestamp: bool,
    prefix: Option<String>,
    colorize: bool,
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

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn colorize(mut self, enabled: bool) -> Self {
        self.colorize = enabled;
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
            if self.colorize {
                eprint!("{} ", ts.dimmed())
            } else {
                eprint!("{} ", ts)
            }
        }

        if let Some(ref p) = self.prefix {
            if self.colorize {
                eprint!("{} ", p.bold());
            } else {
                eprint!("{} ", p)
            }
        }

        let level_str = format!("{:<5}", level.as_str());
        let level_str = if self.colorize {
            match level {
                Level::Trace => level_str.dimmed().to_string(),
                Level::Info => level_str.cyan().to_string(),
                Level::Warn => level_str.yellow().to_string(),
                Level::Debug => level_str.magenta().to_string(),
                Level::Error => level_str.red().to_string(),
            }
        } else {
            level_str
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
            prefix: None,
            colorize: std::io::stderr().is_terminal(), // auto-detect (TTY)
        }
    }
}
