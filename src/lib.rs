#![no_std]

mod js {
	unsafe extern "C" {
		pub(super) fn print(offset: isize, len: isize);
	}
}

fn print(str: &str) {
	unsafe { js::print(str.as_ptr().addr().try_into().unwrap(), str.len().try_into().unwrap()) };
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
	if let Some(message) = info.message().as_str() {
		print(message);
	}
	loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn run() {
	print("Hello, world!");
}
