use once_cell::sync::Lazy;
use std::{
	sync::atomic::{AtomicU64, Ordering::SeqCst}, time::{Instant, UNIX_EPOCH}
};

pub fn timestamp() -> u128 {
	static SYSTEM_TIME_OFFSET: Lazy<u128> = Lazy::new(|| UNIX_EPOCH.elapsed().unwrap().as_micros());
	static MONO_TIME_BASE: Lazy<Instant> = Lazy::new(Instant::now);
	*SYSTEM_TIME_OFFSET + MONO_TIME_BASE.elapsed().as_micros()
}

pub fn process_id() -> u64 {
	std::process::id() as u64
}

pub fn thread_id() -> u64 {
	thread_local! {
		static THREAD_ID: AtomicU64 = AtomicU64::new(0);
	}
	THREAD_ID.with(|thread_id| {
		let cached = thread_id.load(SeqCst);
		if cached != 0 {
			cached
		} else {
			thread_id.compare_and_swap(0, new_virtual_thread_id(), SeqCst);
			thread_id.load(SeqCst)
		}
	})
}

pub fn new_virtual_thread_id() -> u64 {
	static COUNTER: AtomicU64 = AtomicU64::new(1);
	COUNTER.fetch_add(1, SeqCst)
}
