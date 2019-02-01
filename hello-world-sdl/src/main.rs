#![no_std]
#![no_main]

extern crate ndless_handler;

use core::cmp;

use ndless_sdl::video::{SurfaceFlag, VideoFlag};
use nspire::prelude::*;

#[no_mangle]
fn main() {
	ndless_sdl::init(&[ndless_sdl::InitFlag::Video]);
	let screen = match ndless_sdl::video::set_video_mode(320, 240, 16,
	                                                     &[SurfaceFlag::SWSurface],
	                                                     &[VideoFlag::NoFrame]) {
		Ok(screen) => screen,
		Err(err) => panic!("failed to set video mode: {}", err)
	};
	// TODO: free font
	let font = unsafe { ndless_sdl::nSDL_LoadFont(0, 255, 255, 255) };
	let mut j = 0u64;
	'main: loop {
		for i in 0usize..10 {
			for j in 0usize..10 {
				screen.fill_rect(Some(ndless_sdl::Rect {
					x: (i as i16) * 800 / 10,
					y: (j as i16) * 600 / 10,
					w: 800 / 10,
					h: 600 / 10
				}), ndless_sdl::video::RGB(142, 120, 255));
			}
		}
		let keys = nspire::input::get_keys();
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
		j += 1;
		screen.flip();
	}
	ndless_sdl::quit();
}
