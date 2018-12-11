/* util-NOT v0.0.1
*  This file is part of utils-bitwise, written by Ellie Martin-Eberhardt.
*  You may use and distribute this file under the terms of the AGPL v3 license,
*  a copy of which should be included whenever this file or its package are
*  distributed.
*  This file comes with absolutely no warranty or promise of usefulness for any
*  purpose. Anything you make with utils-bitwise is probably going to be weird
*  and cursed, and that's great, but you definitely can't blame me for whatever
*  happens because of it.
*/

use std::env::args;
use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;
use std::fs::File;
use std::process::exit;

fn main() {
	let arguments:Vec<String> = args().collect();
	if arguments.len() > 2 {
		exit(0);
	}
	if arguments.len() == 2 && arguments[1] != String::from("-") {
		let mut byte_buffer:[u8;1] = [0];
		let mut input_file:File = match File::open(&arguments[1]) {
			Err(why) => {
				eprintln!("not: could not open {}: {}",&arguments[1],why);
				exit(1);
			},
			Ok(file) => file,
		};
		loop {
			match input_file.read(&mut byte_buffer) {
				Err(_why) => {
					exit(1);
				},
				Ok(0) => exit(0),
				Ok(_) => (),
			};
			byte_buffer[0] = !byte_buffer[0];
			match stdout().write_all(&byte_buffer) {
				Err(_why) => {
					exit(1);
				},
				Ok(_) => (),
			};
		}
	} else {
		let mut byte_buffer:[u8;1] = [0];
		loop {
			match stdin().read(&mut byte_buffer) {
				Err(_why) => {
					exit(1);
				},
				Ok(0) => exit(0),
				Ok(_) => (),
			};
			byte_buffer[0] = !byte_buffer[0];
			match stdout().write_all(&byte_buffer) {
				Err(_why) => {
					exit(1);
				},
				Ok(_) => (),
			};
		}
	}
}
