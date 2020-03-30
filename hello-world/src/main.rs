#![no_std]
#![no_main]

extern crate ndless_handler;

use ndless::prelude::*;

#[entry]
fn main() {
	use ndless::msg::*;
	let button_pressed = msg_3b("Hello", "Hello, World!", "1", "2", "3");
	let message = match button_pressed {
		Button::One => "one",
		Button::Two => "two",
		Button::Three => "three",
	};
	msg("Title", &format!("You pressed button number {}!", message));
}
