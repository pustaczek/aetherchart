use crate::raw::{Event, Kind};
use once_cell::sync::Lazy;
use serde_json::{json, Value as Json};
use std::{
	sync::atomic::{AtomicU64, Ordering::SeqCst}, time::{Instant as MonoTime, UNIX_EPOCH}
};

static SYSTEM_TIME_OFFSET: Lazy<u128> = Lazy::new(|| UNIX_EPOCH.elapsed().unwrap().as_micros());
static MONO_TIME_BASE: Lazy<MonoTime> = Lazy::new(MonoTime::now);

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
		make_event(kind, Some(self.category), self.name, None, json!({}))
	}
}

impl<'e> Instant<'e> {
	pub fn event(&self) -> Event<'e> {
		let (name, scope) = match self {
			Instant::Thread { name } => (name, "t"),
			Instant::Process { name } => (name, "p"),
			Instant::Global { name } => (name, "g"),
		};
		make_event(Kind::Instant, None, name, Some(scope), json!({}))
	}
}

impl<'e> Metadata<'e> {
	pub fn event(&self) -> Event<'e> {
		let (name, args) = match self {
			Metadata::ProcessName { name } => ("process_name", json!({ "name": name })),
			Metadata::ThreadName { name } => ("thread_name", json!({ "name": name })),
		};
		make_event(Kind::Metadata, None, name, None, args)
	}
}

fn make_event<'e>(
	kind: Kind,
	category: Option<&'e str>,
	name: &'e str,
	scope: Option<&'e str>,
	args: Json,
) -> Event<'e>
{
	Event {
		process_id: std::process::id() as u64,
		thread_id: thread_id(),
		timestamp: *SYSTEM_TIME_OFFSET + MONO_TIME_BASE.elapsed().as_micros(),
		kind,
		category,
		name,
		scope,
		args,
	}
}

fn thread_id() -> u64 {
	static COUNTER: AtomicU64 = AtomicU64::new(1);
	thread_local! {
		static THREAD_ID: AtomicU64 = AtomicU64::new(0);
	}
	THREAD_ID.with(|thread_id| {
		let cached = thread_id.load(SeqCst);
		if cached != 0 {
			cached
		} else {
			thread_id.compare_and_swap(0, COUNTER.fetch_add(1, SeqCst), SeqCst);
			thread_id.load(SeqCst)
		}
	})
}
