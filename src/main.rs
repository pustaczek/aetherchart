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
	/// Timestamp, measured in microseconds. The offset is nonzero, but it's aligned neither with
	/// Chrome launch nor with any special hour. It's also related to other fields, because
	/// changing the offset to 0 removes all JS function calls from profiler graph.
	#[serde(rename = "ts")]
	timestamp: i64,
	/// All profiler chunks have this set to `P`.
	ph: &'a str,
	/// JS funcions have `disabled-by-default-v8.cpu_profiler`.
	#[serde(rename = "cat")]
	category: &'a str,
	/// Entries that contain samples and stack trace information have this set to `ProfileChunk`.
	/// This likely indicates the general type of an event. Instead of having args as an untagged
	/// enum, it could be set to externally tagged with this, but that would require reading all
	/// other variants' schemas.
	name: &'a str,
	#[serde(skip_serializing_if = "Option::is_none")]
	dur: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	tdur: Option<i64>,
	tts: i64,
	#[serde(skip_serializing_if = "Option::is_none")]
	id: Option<&'a str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	s: Option<&'a str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	bind_id: Option<&'a str>,
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
	/// Function name can be qualified, or in case of regexes, even contain escaped characters.
	#[serde(rename = "functionName")]
	function_name: Cow<'a, str>,
	/// Contains a (possibly always absolute) URL to source code. Can be set to any https
	/// resources, but file resources cause Chrome to open an empty tab.
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
	/// Frame id is not unique across the entire saved profile. There seem to be 3 different frames
	/// with IDs 1 and 2 in the test data.
	id: i64,
	/// The identifier of parent frame, as seen in timeline flamechart.
	#[serde(skip_serializing_if = "Option::is_none")]
	parent: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CpuProfilePC<'a> {
	/// Contains metadata about frames present around the chunk. Frames do seem to be unique in
	/// their task context, but I'm unsure what controls their placement.
	#[serde(borrow, skip_serializing_if = "Option::is_none")]
	nodes: Option<Vec<NodePC<'a>>>,
	/// Contains ids of sampled frames in about 0.16ms, giving a sample rate of 6250 Hz.
	samples: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataPC<'a> {
	#[serde(borrow, rename = "cpuProfile")]
	cpu_profile: CpuProfilePC<'a>,
	/// Contains time deltas between collected samples present in cpu_profile field. Usually close
	/// to 0.16ms.
	#[serde(rename = "timeDeltas")]
	time_deltas: Vec<i64>,
	/// Possibly which lines in these functions were being executed when the sampling happened?
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
	let profile = std::fs::read_to_string("experiments/profile.json")?;
	let mut profile: Vec<Event> = serde_json::from_str(&profile)?;
	for ev in &mut profile {
		if let Args::ProfileChunk(pc) = &mut ev.args {
			if let Some(ns) = &mut pc.data.cpu_profile.nodes {
				for n in ns {
					if n.call_frame.function_name == "window.updateTypography" {
						n.call_frame.url = Some("https://rocket.rs".into());
					}
				}
			}
		}
	}
	std::fs::write("yoyo.json", serde_json::to_string(&profile)?)?;
	Ok(())
}
