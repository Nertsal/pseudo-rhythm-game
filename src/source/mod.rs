use rodio::Source;
use std::time::Duration;

mod beat;
mod raw;

pub use beat::Beat;
pub use raw::{IntoRawSource, RawSource};
