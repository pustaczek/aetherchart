use once_cell::sync::Lazy;

// Must be a macro because of Rust issue #57563, which prevents const fn from ever interacting with
// function pointers.
#[macro_export]
macro_rules! virtual_thread_init {
	() => {
		$crate::VirtualThread { id: $crate::Lazy::new($crate::new_virtual_thread_id) };
	};
}

#[allow(dead_code)]
pub struct VirtualThread {
	#[doc(hidden)]
	pub id: Lazy<u64>,
}

impl VirtualThread {
	pub fn id(&self) -> u64 {
		*self.id
	}
}
