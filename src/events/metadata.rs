use crate::{central::CENTRAL, events::Event, os};
use std::borrow::Cow;

pub fn track_process_name(name: impl Into<Cow<'static, str>>) {
	CENTRAL.send(Event::Metadata(Metadata {
		name: name.into(),
		timestamp: os::timestamp(),
		process_id: os::process_id(),
		thread_id: os::thread_id(),
		scope: Scope::Process,
	}))
}

pub fn track_thread_name(name: impl Into<Cow<'static, str>>) {
	track_thread_name_ext(name).emit()
}

pub fn track_thread_name_ext(name: impl Into<Cow<'static, str>>) -> Metadata {
	Metadata {
		name: name.into(),
		timestamp: 0,
		process_id: os::process_id(),
		thread_id: os::thread_id(),
		scope: Scope::Thread,
	}
}

#[must_use = "call `.emit()` to register the event"]
pub struct Metadata {
	pub(crate) name: Cow<'static, str>,
	pub(crate) timestamp: u128,
	pub(crate) process_id: u64,
	pub(crate) thread_id: u64,
	pub(crate) scope: Scope,
}

pub enum Scope {
	Process,
	Thread,
}

impl Metadata {
	pub fn override_thread(self, thread_id: u64) -> Metadata {
		Metadata { thread_id, ..self }
	}

	pub fn emit(mut self) {
		self.timestamp = os::timestamp();
		CENTRAL.send(Event::Metadata(Metadata { ..self }));
	}
}
