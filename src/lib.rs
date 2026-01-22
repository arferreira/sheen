pub mod global;
mod level;
mod logger;
mod macros;

pub use global::{info, init};
pub use level::Level;
pub use logger::Logger;
