#![no_std]

//! This demonstrates the basic usage of SDL and FreeType.

extern crate ndless_handler;

use ndless::input::{get_keys, Key};
use ndless::prelude::*;
use ndless_sdl::text::freetype::Text;

fn main() {
	let library = ndless_freetype::Library::init().unwrap();
	// See https://stackoverflow.com/a/44440992 for how the reduced file was generated
	// Also turn off "PS Glyph Names" in FontForge
	let open_sans = Vec::from(&include_bytes!("Roboto-Light-reduced.ttf")[..]);
	let face = library.new_memory_face(open_sans, 0).unwrap();

	let screen = ndless_sdl::init_default().expect("failed to set video mode");

	screen.fill_rect(
		Some(ndless_sdl::Rect {
			x: 0,
			y: 0,
			w: 320,
			h: 240,
		}),
		ndless_sdl::video::RGB(142, 120, 255),
	);
	let mut text = Text::new(&face);
	text.text("this is some text").height(40);
	screen.blit(&text.render());
	text.reallocate();
	screen.flip();
	ndless::input::wait_key_pressed();
	let mut i = 0;
	loop {
		if get_keys().contains(&Key::Esc) {
			break;
		}
		screen.fill_rect(
			Some(ndless_sdl::Rect {
				x: 0,
				y: 0,
				w: 320,
				h: 240,
			}),
			ndless_sdl::video::RGB(142, 120, 255),
		);
		i += 1;
		text.text(format!("iter {}", i));
		screen.blit(&text.render());

		screen.flip();
	}
	ndless_sdl::quit();
}
