use crate::track_thread_name_ext;
use once_cell::sync::{Lazy, OnceCell};

// Must be a macro because of Rust issue #57563, which prevents const fn from ever interacting with
// function pointers.
#[macro_export]
macro_rules! virtual_thread_init {
	($name:expr) => {
		$crate::VirtualThread {
			id: $crate::Lazy::new($crate::new_virtual_thread_id),
			was_initialized: $crate::OnceCell::new(),
			name: $name,
			}
	};
}

#[allow(dead_code)]
pub struct VirtualThread {
	#[doc(hidden)]
	pub id: Lazy<u64>,
	#[doc(hidden)]
	pub was_initialized: OnceCell<()>,
	#[doc(hidden)]
	pub name: &'static str,
}

impl VirtualThread {
	pub fn id(&self) -> u64 {
		let id = *self.id;
		if self.was_initialized.set(()).is_ok() {
			track_thread_name_ext(self.name).override_thread(id).emit();
		}
		id
	}
}
