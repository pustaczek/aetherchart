use crate::Event;
use once_cell::sync::Lazy;
use std::{path::Path, sync::Mutex};

pub static CENTRAL: Central = Central { inner: Lazy::new(|| Mutex::new(Vec::new())) };

pub struct Central {
	inner: Lazy<Mutex<Vec<Event<'static>>>>,
}

impl Central {
	pub fn send(&self, event: Event<'static>) {
		self.inner.lock().unwrap().push(event);
	}

	pub fn drain(&self) -> Vec<Event<'static>> {
		std::mem::replace(&mut self.inner.lock().unwrap(), Vec::new())
	}

	pub fn save_to(&self, path: impl AsRef<Path>) {
		let file = std::fs::File::create(path).unwrap();
		let events = self.drain();
		serde_json::to_writer(file, &events).unwrap();
	}
}
