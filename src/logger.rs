use chrono::Local;
use std::{fmt::Debug, io::IsTerminal};

use crate::{Formatter, Level, formatter::TextFormatter};

pub struct Logger {
    level: Level,
    show_timestamp: bool,
    prefix: Option<String>,
    colorize: bool,
    fields: Vec<(String, String)>,
    formatter: Box<dyn Formatter>,
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

    pub fn formatter<F: Formatter + 'static>(mut self, f: F) -> Self {
        self.formatter = Box::new(f);
        self
    }

    pub fn with(&self, fields: &[(&str, &dyn std::fmt::Debug)]) -> Self {
        let mut new_fields = self.fields.clone();
        for (key, value) in fields {
            new_fields.push((key.to_string(), format!("{:?}", value)));
        }

        // returns a new Logger, doesn't mutate original
        Self {
            level: self.level,
            show_timestamp: self.show_timestamp,
            prefix: self.prefix.clone(),
            colorize: std::io::stderr().is_terminal(),
            fields: new_fields,
            formatter: Box::new(TextFormatter::new(self.colorize)),
        }
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

        let timestamp = if self.show_timestamp {
            Some(Local::now().format("%H:%M:%S").to_string())
        } else {
            None
        };

        let output = self.formatter.format(
            level,
            message,
            timestamp.as_deref(),
            self.prefix.as_deref(),
            &self.fields,
            fields,
        );

        eprintln!("{}", output);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            level: Level::Info,
            show_timestamp: true,
            prefix: None,
            colorize: std::io::stderr().is_terminal(), // auto-detect (TTY)
            fields: Vec::new(),
            formatter: Box::new(TextFormatter::new(std::io::stderr().is_terminal())),
        }
    }
}

#[cfg(feature = "log")]
impl Logger {
    pub fn init(self) -> Result<(), log::SetLoggerError> {
        let max_level: log::LevelFilter = self.level.into();
        log::set_boxed_logger(Box::new(self))?;
        log::set_max_level(max_level);
        Ok(())
    }
}
