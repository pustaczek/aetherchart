pub mod duration;
pub mod metadata;

pub enum Event {
	Duration(duration::Duration),
	Metadata(metadata::Metadata),
}

impl Event {
	pub(crate) fn pid_and_tid(&self) -> (u64, u64) {
		match self {
			Event::Duration(ev) => (ev.process_id, ev.thread_id),
			Event::Metadata(ev) => (ev.process_id, ev.thread_id),
		}
	}

	pub(crate) fn timestamp(&self) -> u128 {
		match self {
			Event::Duration(ev) => ev.timestamp,
			Event::Metadata(ev) => ev.timestamp,
		}
	}
}
