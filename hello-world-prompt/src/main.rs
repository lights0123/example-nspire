#![no_std]
#![no_main]

extern crate ndless_handler;

use ndless::prelude::*;

#[entry]
fn main() {
	use ndless::msg::*;
	match msg_numeric("title", "subtitle", "msg", (0, 5)) {
		None => msg("result", "You cancelled the operation!"),
		Some(num) => msg("result", &format!("You typed {}!", num)),
	}
	match msg_2numeric("title", "subtitle", "msg1", (0, 5), "msg2", (6, 10)) {
		None => msg("result", "You cancelled the operation!"),
		Some((num1, num2)) => msg("result", &format!("You typed {} and {}!", num1, num2)),
	}
	match msg_input("title", "subtitle", "msg") {
		None => msg("result", "You cancelled the operation!"),
		Some(response) => msg("result", &format!("You typed \"{}\"!", response)),
	}
}
