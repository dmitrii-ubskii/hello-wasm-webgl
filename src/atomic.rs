use core::sync::atomic::{AtomicU32, Ordering};

pub(crate) struct AtomicF32Array<const N: usize> {
	pub(crate) inner: [AtomicU32; N],
}

impl<const N: usize> AtomicF32Array<N> {
	pub(crate) const fn new(array: [f32; N]) -> Self {
		let mut inner = [const { AtomicU32::new(0) }; N];
		let mut i = 0;
		loop {
			inner[i] = AtomicU32::new(array[i].to_bits());
			i += 1;
			if i == N {
				break;
			}
		}
		Self { inner }
	}

	pub(crate) fn store(&self, i: usize, x: f32) {
		self.inner[i].store(x.to_bits(), Ordering::Relaxed);
	}

	pub(crate) fn as_ptr(&self) -> *const f32 {
		self.inner.as_ptr().cast()
	}
}
