use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize)]
struct Event<'a> {
	/// On test data, happens to be `16614` on main thread, `16663` on thread 1, `16743` on frame
	/// thread.
	#[serde(rename = "pid")]
	process_id: i64,
	/// On test data, happens to be `12` on main thread and thread 1, but `11` on frame thread.
	#[serde(rename = "tid")]
	thread_id: i64,
	#[serde(rename = "ts")]
	timestamp: i64,
	ph: Cow<'a, str>,
	/// JS funcions have `disabled-by-default-v8.cpu_profiler`.
	#[serde(rename = "cat")]
	category: Cow<'a, str>,
	/// JS functions during profiling get a `ProfileChunk` name. These events contain entire lists
	/// of grouped events, coupled with parent-child relationships between various frames. I can't
	/// understand how does it tell when the function ends, but I guess it's from the samples.
	name: Cow<'a, str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	dur: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	tdur: Option<i64>,
	tts: i64,
	#[serde(skip_serializing_if = "Option::is_none")]
	id: Option<Cow<'a, str>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	s: Option<Cow<'a, str>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	bind_id: Option<Cow<'a, str>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	flow_in: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	flow_out: Option<bool>,
	/// Most profiling information is contained in this field.
	#[serde(borrow)]
	args: Args<'a>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CallFramePC<'a> {
	#[serde(rename = "functionName")]
	function_name: Cow<'a, str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	url: Option<Cow<'a, str>>,
	#[serde(rename = "scriptId")]
	script_id: i64,
	#[serde(rename = "lineNumber", skip_serializing_if = "Option::is_none")]
	line_number: Option<i64>,
	#[serde(rename = "columnNumber", skip_serializing_if = "Option::is_none")]
	column_number: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NodePC<'a> {
	#[serde(borrow, rename = "callFrame")]
	call_frame: CallFramePC<'a>,
	id: i64,
	#[serde(skip_serializing_if = "Option::is_none")]
	parent: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CpuProfilePC<'a> {
	#[serde(borrow, skip_serializing_if = "Option::is_none")]
	nodes: Option<Vec<NodePC<'a>>>,
	samples: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataPC<'a> {
	#[serde(borrow, rename = "cpuProfile")]
	cpu_profile: CpuProfilePC<'a>,
	#[serde(rename = "timeDeltas")]
	time_deltas: Vec<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	lines: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PC<'a> {
	#[serde(borrow)]
	data: DataPC<'a>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Args<'a> {
	#[serde(borrow)]
	ProfileChunk(PC<'a>),
	Any(Json),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let profile = std::fs::read_to_string("profile.json")?;
	let mut profile: Vec<Event> = serde_json::from_str(&profile)?;
	for ev in &mut profile {
		if ev.process_id == 16614 && ev.thread_id == 12 {
			if let Args::ProfileChunk(pc) = &mut ev.args {
				for sample in &mut pc.data.cpu_profile.samples {
					*sample = 816;
				}
			}
		}
	}
	std::fs::write("yoyo.json", serde_json::to_string(&profile)?)?;
	Ok(())
}
