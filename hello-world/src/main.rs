#![no_std]
#![no_main]

use nspire::prelude::*;
extern crate ndless_handler;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32
}
#[no_mangle]
fn main() {
    use nspire::msg::*;
    let a = Rect {
        x: 0,
        y: 0,
        w: 240,
        h: 240
    };
    msg("msg", "msg2");
    msg("msg", format!("{:?}", a));
    msg("msg", format!("{}", a.w));
    let button_pressed = msg_3b("Hello", "Hello, World!", "1", "2", "3");
    let message = match button_pressed {
        Button::ONE => "one",
        Button::TWO => "two",
        Button::THREE => "three",
    };
    msg("Title", format!("You pressed button number {}!", message));
}
