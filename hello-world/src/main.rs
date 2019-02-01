#![no_std]
#![no_main]

use nspire::prelude::*;
extern crate ndless_handler;

#[no_mangle]
fn main() {
    use nspire::msg::*;
    let button_pressed = msg_3b("Hello", "Hello, World!", "1", "2", "3");
    let message = match button_pressed {
        Button::ONE => "one",
        Button::TWO => "two",
        Button::THREE => "three",
    };
    msg("Title", format!("You pressed button number {}!", message));
}
