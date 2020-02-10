use crate::{chrome_dev_tools, events::Event, validate::Validator};
use once_cell::sync::Lazy;
use slog::Logger;
use std::{path::Path, sync::Mutex};

pub static CENTRAL: Central = Central { inner: Lazy::new(|| Mutex::new(Vec::new())) };

pub struct Central {
	inner: Lazy<Mutex<Vec<Event>>>,
}

impl Central {
	pub fn send(&self, event: Event) {
		self.inner.lock().unwrap().push(event);
	}

	pub fn save_to(&self, path: impl AsRef<Path>, log: &Logger) {
		let file = std::fs::File::create(path).unwrap();
		let events = self.drain(log);
		let events = events.into_iter().map(chrome_dev_tools::Event::from).collect::<Vec<_>>();
		serde_json::to_writer(file, &events).unwrap();
	}

	fn drain(&self, log: &Logger) -> Vec<Event> {
		let events = std::mem::replace(&mut *self.inner.lock().unwrap(), Vec::new());
		let validator = Validator::new(&events, log);
		validator.validate();
		events
	}
}
