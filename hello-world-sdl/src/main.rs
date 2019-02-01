#![no_std]
#![no_main]

extern crate ndless_handler;

use core::cmp;

use ndless_sdl::video::{SurfaceFlag, VideoFlag};
use nspire::prelude::*;
#[derive(PartialEq, Copy, Clone)]
pub struct Rect {
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32
}
#[no_mangle]
fn main() {
	let a = Rect {
		x: 0,
		y: 0,
		w: 240,
		h: 240
	};
	nspire::msg::msg("msg", format!("{}", a.w));
	let button_pressed = nspire::msg::msg_3b("Hello", "Hello, World!", "1", "2", "3");
	let message = match button_pressed {
		nspire::msg::Button::ONE => "one",
		nspire::msg::Button::TWO => "two",
		nspire::msg::Button::THREE => "three",
	};
	nspire::msg::msg("Title", format!("You pressed button number {}!", message));
	ndless_sdl::init(&[ndless_sdl::InitFlag::Video]);
	let screen = match ndless_sdl::video::set_video_mode(320, 240, 16,
	                                                     &[SurfaceFlag::SWSurface],
	                                                     &[VideoFlag::NoFrame]) {
		Ok(screen) => screen,
		Err(err) => panic!("failed to set video mode: {}", err)
	};
	let font = unsafe { ndless_sdl::nSDL_LoadFont(0, 255, 255, 255) };
	unsafe { ndless_sys::msleep(1000); }
//	let mut j = 0u32;
	'main: loop {
		/*for i in 0usize..10 {
			for j in 0usize..10 {
				(Some(ndless_sdl::Rect {
					x: 0,
					y: 0,
					w: 240,
					h: 240
				}), ndless_sdl::video::RGB(142, 120, 255));
			}
		}*/
		/*let keys = nspire::input::get_keys();
		if keys.contains(&nspire::input::Key::KEY_NSPIRE_ESC) { break; }
		let mut message = format!("{:?}", keys);
		let mut i = 0;
		while message.len() > 0 {
			let len = cmp::min(message.len(), 40);
			let msg = nspire::cstr!(&message[..len]);
			unsafe {
				ndless_sdl::nSDL_DrawString(screen.raw, font, 0, i, msg.as_ptr());
			}
			message = String::from(&message[len..]);
			i += 10;
		}
		let msg = nspire::cstr!(format!("{}", j));
		unsafe {
			ndless_sdl::nSDL_DrawString(screen.raw, font, 0, 100, msg.as_ptr());
		}
		j += 1;*/
//		j += 1;
		/*let msg = nspire::cstr!(format!("{}", j));
		unsafe {
			ndless_sdl::nSDL_DrawString(screen.raw, font, 0, 100, msg.as_ptr());
		}*/
		screen.flip();
	}
	ndless_sdl::quit();
}
