#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate ndless_handler;

use nspire::prelude::*;
use core::fmt::Write;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect {
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32
}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	let msg = match info.message() {
		Some(err) => format!("An error occured: {}", err),
		None => format!("An error occured!")
	};
	nspire::msg::msg("Error", format!("{}", msg));
	unsafe { ndless_sys::abort() }
}
#[no_mangle]
fn format(args: core::fmt::Arguments, str: &mut String) {
	str.write_fmt(args);
}
#[no_mangle]
fn main() {
	use nspire::msg::*;
	let mut a = Rect {
		x: 0,
		y: 0,
		w: 240,
		h: 250
	};
	let mut b = String::new();
	msg("msg", "msg");
	format(format_args!("{}", a.w), &mut b);
	msg("msg", "msg2");
}
