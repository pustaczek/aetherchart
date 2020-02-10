use crate::{central::CENTRAL, events::Event, os};
use std::{borrow::Cow, mem};

pub fn track_duration(
	name: impl Into<Cow<'static, str>>,
	category: impl Into<Cow<'static, str>>,
) -> DurationGuard
{
	track_duration_ext(name, category).guard()
}

pub fn track_duration_ext(
	name: impl Into<Cow<'static, str>>,
	category: impl Into<Cow<'static, str>>,
) -> Duration
{
	Duration {
		name: name.into(),
		category: category.into(),
		timestamp: 0,
		process_id: os::process_id(),
		thread_id: os::thread_id(),
		start: true,
	}
}

#[must_use = "call `.guard()` to start the duration and return a RAII guard"]
pub struct Duration {
	pub(crate) name: Cow<'static, str>,
	pub(crate) category: Cow<'static, str>,
	pub(crate) timestamp: u128,
	pub(crate) process_id: u64,
	pub(crate) thread_id: u64,
	pub(crate) start: bool,
}

pub struct DurationGuard {
	duration: Duration,
}

impl Duration {
	pub fn override_thread(self, thread_id: u64) -> Duration {
		Duration { thread_id, ..self }
	}

	pub fn guard(mut self) -> DurationGuard {
		self.timestamp = os::timestamp();
		CENTRAL.send(Event::Duration(Duration {
			name: self.name.clone(),
			category: self.category.clone(),
			start: true,
			..self
		}));
		DurationGuard { duration: self }
	}
}

impl Drop for DurationGuard {
	fn drop(&mut self) {
		self.duration.timestamp = os::timestamp();
		CENTRAL.send(Event::Duration(Duration {
			name: mem::replace(&mut self.duration.name, Cow::Owned(String::new())),
			category: mem::replace(&mut self.duration.category, Cow::Owned(String::new())),
			start: false,
			..self.duration
		}))
	}
}
