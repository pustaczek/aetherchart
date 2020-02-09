use aetherchart::{Duration, Event, Metadata};
use std::fs::File;

fn write_events(events: &[Event]) {
	serde_json::to_writer(File::create("yoyo.json").unwrap(), &events).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let metadata = Metadata::ThreadName { name: "Main thread" };
	let parse_css = Duration::new("Parse CSS", "parse");
	let paint_tree = Duration::new("Paint Tree", "paint");
	let events = [
		metadata.event(),
		parse_css.start(),
		paint_tree.start(),
		paint_tree.end(),
		parse_css.end(),
	];
	write_events(&events);
	Ok(())
}
