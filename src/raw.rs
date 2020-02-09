use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub enum Kind {
	#[serde(rename = "B")]
	Begin,
	#[serde(rename = "E")]
	End,
	#[serde(rename = "M")]
	Metadata,
}

#[derive(Deserialize, Serialize)]
pub struct Event<'a> {
	#[serde(rename = "pid")]
	pub process_id: u64,
	#[serde(rename = "tid")]
	pub thread_id: u64,
	#[serde(rename = "ts")]
	pub timestamp: u64,
	#[serde(rename = "ph")]
	pub kind: Kind,
	#[serde(rename = "cat")]
	pub category: &'a str,
	pub name: &'a str,
	pub args: Value,
}
