#![feature(lang_items)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::alloc::{ Layout};
use core::panic::PanicInfo;

use ndless_sys::{wait_key_pressed};
use nspire::prelude::*;

#[lang = "eh_personality"]
extern fn eh_personality() {}

extern "C" {
    //	fn wait_key_pressed();
//	fn _show_msgbox(title: *const u8, message: *const u8, buttons: u32) -> u32;
    fn SDL_Init(flags: u32) -> i32;
    fn abort() -> !;
}

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    unsafe { abort(); }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { abort() }
}

#[no_mangle]
pub extern "C" fn main() {
    nspire::msg::msg("hi", format!("hi {:?}", nspire::msg::msg_2b("t", "m", "1", "2") as u32));

    unsafe {
//		SDL_Init(0x00000020);
        wait_key_pressed();
    }
}
