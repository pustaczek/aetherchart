use crate::{raw::Kind, Event};
use slog::{warn, Logger};
use std::collections::HashMap;

pub struct Validator<'a> {
	threads: HashMap<(u64, u64), Vec<&'a Event<'a>>>,
	log: &'a Logger,
}

impl<'a> Validator<'a> {
	pub fn new(events: &'a [Event<'a>], log: &'a Logger) -> Validator<'a> {
		let mut threads = HashMap::<_, Vec<&'a Event<'a>>>::new();
		for event in events {
			let thread = (event.process_id, event.thread_id);
			threads.entry(thread).or_default().push(event);
		}
		Validator { threads, log }
	}

	pub fn validate(&self) {
		self.validate_durations();
		self.validate_order();
	}

	fn validate_durations(&self) {
		for (thread, events) in &self.threads {
			let mut stack = Vec::new();
			for event in events {
				match event.kind {
					Kind::Begin => stack.push(*event),
					Kind::End => {
						let frame = match stack.last() {
							Some(frame) => *frame,
							None => {
								self.warn("popping durations with empty stack", thread);
								break;
							},
						};
						if (frame.name, frame.category) != (event.name, event.category) {
							self.warn("interleaving durations", thread);
							break;
						}
						stack.pop();
					},
					_ => (),
				}
			}
		}
	}

	fn validate_order(&self) {
		for (thread, events) in &self.threads {
			let is_ordered =
				(0..events.len() - 1).all(|i| events[i].timestamp <= events[i + 1].timestamp);
			if !is_ordered {
				self.warn("events are ordered incorrectly", thread);
			}
		}
	}

	fn warn(&self, msg: &str, thread: &(u64, u64)) {
		warn!(
			self.log,
			"aetherchart malformed, {}",
			msg;
			"thread" => format_args!("{:?}", thread)
		)
	}
}
