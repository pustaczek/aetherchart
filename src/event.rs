use crate::raw::{Event, Kind};
use once_cell::sync::Lazy;
use serde_json::{json, Value as Json};
use std::time::Instant;

static EPOCH: Lazy<Instant> = Lazy::new(Instant::now);

pub struct Duration<'e> {
	name: &'e str,
	category: &'e str,
}

pub enum Metadata<'e> {
	ProcessName { name: &'e str },
	// ProcessLabels { labels: _ },
	// ProcessSortIndex { sort_index: _ },
	ThreadName { name: &'e str },
	// ThreadSortIndex { sort_index: _ },
}

impl<'e> Duration<'e> {
	pub fn new(name: &'e str, category: &'e str) -> Duration<'e> {
		Duration { name, category }
	}

	pub fn start(&self) -> Event<'e> {
		self.make_event(Kind::Begin)
	}

	pub fn end(&self) -> Event<'e> {
		self.make_event(Kind::End)
	}

	fn make_event(&self, kind: Kind) -> Event<'e> {
		make_event(kind, self.category, self.name, json!({}))
	}
}

impl<'e> Metadata<'e> {
	pub fn event(&self) -> Event<'e> {
		let (name, args) = match self {
			Metadata::ProcessName { name } => ("process_name", json!({ "name": name })),
			Metadata::ThreadName { name } => ("thread_name", json!({ "name": name })),
		};
		make_event(Kind::Metadata, "", name, args)
	}
}

fn make_event<'e>(kind: Kind, category: &'e str, name: &'e str, args: Json) -> Event<'e> {
	Event {
		process_id: std::process::id() as u64,
		thread_id: std::thread::current().id().as_u64(),
		timestamp: EPOCH.elapsed().as_micros() as u64,
		kind,
		category,
		name,
		args,
	}
}
