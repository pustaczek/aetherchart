use crate::central::CENTRAL;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub enum Kind {
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
	pub kind: Kind,
	#[serde(rename = "cat", skip_serializing_if = "Option::is_none")]
	pub category: Option<&'a str>,
	pub name: &'a str,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope: Option<&'a str>,
	pub args: Value,
}

impl Event<'static> {
	pub fn send(self) {
		CENTRAL.send(self);
	}
}
