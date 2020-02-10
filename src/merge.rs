use aetherchart::chrome_dev_tools;
use std::io::stdout;

fn main() {
	let contents = std::env::args()
		.skip(1)
		.map(|path| std::fs::read_to_string(path).unwrap())
		.collect::<Vec<_>>();
	let mut events = contents
		.iter()
		.flat_map(|file| {
			let events: Vec<chrome_dev_tools::Event> = serde_json::from_str(&file).unwrap();
			events.into_iter()
		})
		.collect::<Vec<_>>();
	let min_timestamp = events.iter().map(|ev| ev.timestamp).min();
	if let Some(min_timestamp) = min_timestamp {
		for event in &mut events {
			event.timestamp -= min_timestamp - 1;
		}
	}
	println!("[");
	for i in 0..events.len() {
		if i > 0 {
			println!(",");
		}
		serde_json::to_writer(stdout(), &events[i]).unwrap();
	}
	println!("\n]");
}
