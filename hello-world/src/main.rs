#![no_std]
#![no_main]

extern crate ndless_handler;

use ndless::prelude::*;

#[entry]
fn main() {
	use ndless::msg::*;
	let button_pressed = msg_3b("Hello", "Hello, World!", "1", "2", "3");
	let message = match button_pressed {
		Button::ONE => "one",
		Button::TWO => "two",
		Button::THREE => "three",
	};
	msg("Title", format!("You pressed button number {}!", message));
}
