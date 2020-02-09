use aetherchart::{Duration, Event, Instant, Metadata};
use std::fs::File;

fn write_events(events: &[Event]) {
	serde_json::to_writer(File::create("yoyo.json").unwrap(), &events).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let metadata = Metadata::ThreadName { name: "Main thread" };
	let parse_css = Duration { name: "Parse CSS", category: "parse" };
	let paint_tree = Duration { name: "Paint Tree", category: "paint" };
	let finished_draw = Instant::Global { name: "PTF" };
	let events = [
		metadata.event(),
		parse_css.start(),
		paint_tree.start(),
		paint_tree.end(),
		finished_draw.event(),
		parse_css.end(),
	];
	write_events(&events);
	Ok(())
}
