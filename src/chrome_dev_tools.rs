use crate::events::{metadata, Event as AEEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::borrow::Cow;

#[derive(Deserialize, Serialize)]
pub enum EventKind {
	#[serde(rename = "B")]
	Begin,
	#[serde(rename = "E")]
	End,
	#[serde(rename = "i")]
	Instant,
	#[serde(rename = "R")]
	Mark,
	#[serde(rename = "M")]
	Metadata,
}

#[derive(Deserialize, Serialize)]
#[must_use]
pub struct Event<'a> {
	#[serde(rename = "pid")]
	pub process_id: u64,
	#[serde(rename = "tid")]
	pub thread_id: u64,
	#[serde(rename = "ts")]
	pub timestamp: u128,
	#[serde(rename = "ph")]
	pub kind: EventKind,
	#[serde(borrow, rename = "cat", skip_serializing_if = "Option::is_none")]
	pub category: Option<Cow<'a, str>>,
	#[serde(borrow)]
	pub name: Cow<'a, str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope: Option<&'a str>,
	pub args: Value,
}

impl From<crate::events::Event> for Event<'static> {
	fn from(ev: crate::events::Event) -> Self {
		match ev {
			AEEvent::Duration(ev) => Event {
				process_id: ev.process_id,
				thread_id: ev.thread_id,
				timestamp: ev.timestamp,
				kind: if ev.start { EventKind::Begin } else { EventKind::End },
				category: Some(ev.category),
				name: ev.name,
				scope: None,
				args: json!({}),
			},
			AEEvent::Metadata(ev) => Event {
				process_id: ev.process_id,
				thread_id: ev.thread_id,
				timestamp: ev.timestamp,
				kind: EventKind::Metadata,
				category: None,
				name: Cow::Borrowed(match ev.scope {
					metadata::Scope::Process => "process_name",
					metadata::Scope::Thread => "thread_name",
				}),
				scope: None,
				args: json!({
					"name": ev.name
				}),
			},
		}
	}
}
