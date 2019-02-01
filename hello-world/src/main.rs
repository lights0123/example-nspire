#![no_std]
#![no_main]

extern crate ndless_handler;

use nspire::prelude::*;
use core::fmt::Write;

pub struct Rect {
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32
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
	msg("msg", "msg3");
	a.w += 5; // Doesn't fail
	msg("msg", "msg4");
	let mut str = String::new();

	// This works and doesn't crash
	/*for x in unsafe { core::mem::transmute::<u32, [u8; 4]>(a.w) }.iter() {
		msg("test", "loop");
		let mut y = *x;
		while y > 0 {
			match y % 10 {
				0 => str.push('0'),
				1 => str.push('1'),
				2 => str.push('2'),
				3 => str.push('3'),
				4 => str.push('4'),
				5 => str.push('5'),
				6 => str.push('6'),
				7 => str.push('7'),
				8 => str.push('8'),
				9 => str.push('9'),
				_ => (),
			}
			y /= 10;
		}
		str.push(',');
		format!()
	}*/
	msg("msg", "msg4.5");
	let str = format_args!("{}", a.h);
	msg("msg", "msg5");
	// This is an expanded format! macro
	let mut b = String::new();
	b.write_fmt(format_args!("{}", a.h)); // Fails here!
	msg("msg", "msg6");
	msg("msg", format!("{}", b));
	let button_pressed = msg_3b("Hello", "Hello, World!", "1", "2", "3");
	let message = match button_pressed {
		Button::ONE => "one",
		Button::TWO => "two",
		Button::THREE => "three",
	};
	msg("Title", format!("You pressed button number {}!", message));
}
