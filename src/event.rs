use crate::raw::{Event, Kind};
use once_cell::sync::Lazy;
use serde_json::{json, Value as Json};
use std::time::Instant as Time;

static EPOCH: Lazy<Time> = Lazy::new(Time::now);

pub struct Duration<'e> {
	pub name: &'e str,
	pub category: &'e str,
}

// FIXME: Does not render at all, even on global. However, JS can add global markers like first
// contentful paint so there must be a way to do this.
pub enum Instant<'e> {
	Thread { name: &'e str },
	Process { name: &'e str },
	Global { name: &'e str },
}

pub enum Metadata<'e> {
	ProcessName { name: &'e str },
	// ProcessLabels { labels: _ },
	// ProcessSortIndex { sort_index: _ },
	ThreadName { name: &'e str },
	// ThreadSortIndex { sort_index: _ },
}

impl<'e> Duration<'e> {
	pub fn start(&self) -> Event<'e> {
		self.make_event(Kind::Begin)
	}

	pub fn end(&self) -> Event<'e> {
		self.make_event(Kind::End)
	}

	fn make_event(&self, kind: Kind) -> Event<'e> {
		make_event(kind, self.category, self.name, None, json!({}))
	}
}

impl<'e> Instant<'e> {
	pub fn event(&self) -> Event<'e> {
		let (name, scope) = match self {
			Instant::Thread { name } => (name, "t"),
			Instant::Process { name } => (name, "p"),
			Instant::Global { name } => (name, "g"),
		};
		make_event(Kind::Instant, "", name, Some(scope), json!({}))
	}
}

impl<'e> Metadata<'e> {
	pub fn event(&self) -> Event<'e> {
		let (name, args) = match self {
			Metadata::ProcessName { name } => ("process_name", json!({ "name": name })),
			Metadata::ThreadName { name } => ("thread_name", json!({ "name": name })),
		};
		make_event(Kind::Metadata, "", name, None, args)
	}
}

fn make_event<'e>(
	kind: Kind,
	category: &'e str,
	name: &'e str,
	scope: Option<&'e str>,
	args: Json,
) -> Event<'e>
{
	Event {
		process_id: std::process::id() as u64,
		thread_id: std::thread::current().id().as_u64(),
		timestamp: EPOCH.elapsed().as_micros() as u64,
		kind,
		category,
		name,
		scope,
		args,
	}
}
