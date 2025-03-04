use core::{
	alloc::{GlobalAlloc, Layout},
	arch::wasm32::{memory_grow, memory_size},
};

pub(crate) struct Alloc;

pub(crate) const PAGE: usize = 64 * 1024;

unsafe impl GlobalAlloc for Alloc {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		assert!(layout.size() <= PAGE);
		let end = memory_size::<0>() * PAGE;
		memory_grow::<0>(1);
		end as _
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
pub(crate) static ALLOC: Alloc = Alloc;
