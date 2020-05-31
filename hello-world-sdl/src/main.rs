#![no_std]

//! This demonstrates the basic usage of SDL, as well as input from the calculator.

extern crate ndless_handler;

use ndless::input::{get_keys, Key};
use ndless::prelude::*;
use ndless_sdl::nsdl::{Font, FontOptions};

fn word_wrap(str: impl Into<String>, line_length: usize) -> String {
	let str = str.into();
	let mut out = String::new();
	let mut i = 0;
	while i < str.len() {
		let to = core::cmp::min(str.len(), i + line_length);
		out += &str[i..to];
		out.push('\n');
		i += line_length;
	}
	out
}

fn main() {
	let screen = ndless_sdl::init_default().expect("failed to set video mode");

	let font = Font::new(FontOptions::Thin, 255, 255, 255);
	let mut j = 0u32;

	loop {
		screen.fill_rect(
			Some(ndless_sdl::Rect {
				x: 0,
				y: 0,
				w: 320,
				h: 240,
			}),
			ndless_sdl::video::RGB(142, 120, 255),
		);

		let keys = get_keys();
		if keys.contains(&Key::Esc) {
			break;
		}
		let message = format!("Hello World! {:?}", keys);
		screen.draw_str(&font, &word_wrap(message, 50), 0, 0);

		// Normally, in Rust, an overflowing integer will cause a `panic!`. To avoid that,
		// use the `wrapping_add` method.
		j = j.wrapping_add(1);
		screen.draw_str(&font, &format!("{}", j), 0, 100);
		screen.flip();
	}
	ndless_sdl::quit();
}
