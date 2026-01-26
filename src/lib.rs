mod formatter;
pub mod global;
mod level;
mod logger;
mod macros;

#[cfg(feature = "log")]
mod log_adapter;

pub use formatter::{Formatter, JsonFormatter, LogFmtFormatter, TextFormatter};
pub use global::{info, init, init_with};
pub use level::Level;
pub use logger::Logger;
