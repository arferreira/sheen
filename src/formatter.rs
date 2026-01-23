use owo_colors::OwoColorize;

use crate::Level;
use std::fmt::Debug;

pub trait Formatter: Send + Sync {
    fn format(
        &self,
        level: Level,
        message: &str,
        timestamp: Option<&str>,
        prefix: Option<&str>,
        fields: &[(String, String)],
        extra: &[(&str, &dyn Debug)],
    ) -> String;
}

pub struct TextFormatter {
    colorize: bool,
}

impl TextFormatter {
    pub fn new(colorize: bool) -> Self {
        Self { colorize }
    }
}

impl Formatter for TextFormatter {
    fn format(
        &self,
        level: Level,
        message: &str,
        timestamp: Option<&str>,
        prefix: Option<&str>,
        fields: &[(String, String)],
        extra: &[(&str, &dyn Debug)],
    ) -> String {
        let mut output = String::new();

        if let Some(ts) = timestamp {
            output.push_str(ts);
            output.push(' ');
        };

        if let Some(p) = prefix {
            output.push_str(p);
            output.push(' ');
        };

        let level_str = format!("{:<5}", level.as_str());
        if self.colorize {
            let colored = match level {
                Level::Trace => level_str.dimmed().to_string(),
                Level::Debug => level_str.magenta().to_string(),
                Level::Info => level_str.cyan().to_string(),
                Level::Warn => level_str.yellow().to_string(),
                Level::Error => level_str.red().to_string(),
            };
            output.push_str(&colored);
        } else {
            output.push_str(&level_str);
        }
        output.push(' ');

        output.push_str(message);

        for (key, value) in fields {
            output.push_str(&format!(" {}={}", key, value));
        }

        for (key, value) in extra {
            output.push_str(&format!(" {}={:?}", key, value));
        }
        output
    }
}

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(
        &self,
        level: Level,
        message: &str,
        timestamp: Option<&str>,
        prefix: Option<&str>,
        fields: &[(String, String)],
        extra: &[(&str, &dyn Debug)],
    ) -> String {
        let mut parts = vec![
            format!("\"level\":\"{}\"", level.as_str().to_lowercase()),
            format!("\"msg\":\"{}\"", message),
        ];

        if let Some(ts) = timestamp {
            parts.push(format!("\"time\":\"{}\"", ts));
        }

        if let Some(p) = prefix {
            parts.push(format!("\"prefix\":\"{}\"", p));
        }

        for (key, value) in fields {
            parts.push(format!("\"{}\":{}", key, value));
        }

        for (key, value) in extra {
            parts.push(format!("\"{}\":{:?}", key, value));
        }

        format!("{{{}}}", parts.join(","))
    }
}

pub struct LogFmtFormatter;

impl Formatter for LogFmtFormatter {
    fn format(
        &self,
        level: Level,
        message: &str,
        timestamp: Option<&str>,
        prefix: Option<&str>,
        fields: &[(String, String)],
        extra: &[(&str, &dyn Debug)],
    ) -> String {
        let mut parts = vec![
            format!("level={}", level.as_str().to_lowercase()),
            format!("msg=\"{}\"", message),
        ];

        if let Some(ts) = timestamp {
            parts.push(format!("time=\"{}\"", ts));
        }

        if let Some(p) = prefix {
            parts.push(format!("prefix=\"{}\"", p));
        }

        for (key, value) in fields {
            parts.push(format!("{}={}", key, value));
        }

        for (key, value) in extra {
            parts.push(format!("{}={:?}", key, value));
        }

        parts.join(" ")
    }
}
