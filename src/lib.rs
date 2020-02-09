#![feature(thread_id_value)]

mod event;
mod raw;

pub use event::{Duration, Metadata};
pub use raw::Event;
