#![no_std]
#![no_main]
extern crate ndless_handler;

use futures_util::future;
use ndless::input::Key;
use ndless::prelude::*;
use ndless_async::keypad::KeypadListener;
use ndless_async::task::{block_on, AsyncListeners};
use ndless_async::{first, StreamExt};

#[entry]
fn main() {
	let listeners = AsyncListeners::new();
	let keypad = KeypadListener::new(&listeners.timer());
	block_on(&listeners, async {
		let _ = listeners.timer().timeout_ms(5000, do_stuff(&keypad)).await;
		listeners.timer().sleep_ms(2000).await;
		first!(do_other_stuff(&listeners), wait_for_esc(&keypad));
	});
	println!("Exit!");
}

async fn wait_for_esc(keypad: &KeypadListener<'_>) {
	keypad
		.stream()
		.filter(|key| future::ready(key.key == Key::Esc))
		.next()
		.await;
}

async fn do_other_stuff(listeners: &AsyncListeners) {
	loop {
		listeners.timer().sleep_ms(1000).await;
		println!("1s!");
	}
}

async fn do_stuff(listeners: &KeypadListener<'_>) {
	use ndless_async::keypad::KeyState::*;
	let mut keypad = listeners.stream();
	while let Some(event) = keypad.next().await {
		println!(
			"Key {:?} was {}",
			event.key,
			if event.state == Released {
				"released"
			} else {
				"pressed"
			}
		);
		print!("Keys currently pressed: ");
		listeners
			.list_keys()
			.iter()
			.for_each(|key| print!("{:?} ", key));
		println!();
	}
}
