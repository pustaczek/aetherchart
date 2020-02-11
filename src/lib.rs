mod central;
pub mod chrome_dev_tools;
mod events;
mod os;
mod validate;
mod virtual_thread;

pub use central::CENTRAL;
pub use events::{
	duration::{track_duration, track_duration_ext}, metadata::{track_process_name, track_thread_name, track_thread_name_ext}, Event
};
pub use virtual_thread::VirtualThread;

#[doc(hidden)]
pub use once_cell::sync::{Lazy, OnceCell};
#[doc(hidden)]
pub use os::new_virtual_thread_id;
