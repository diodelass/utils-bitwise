/* util-NAND v0.0.1
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
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::process::exit;

fn main() {
	let mut files:Vec<File> = vec![];
	let arguments:Vec<String> = args().collect();
	let mut use_stdin:bool = false;
	let mut quit_on_stdin:bool = false;
	if arguments.len() == 1 {
		exit(0);
	}
	for n in 1..arguments.len() {
		if arguments[n] == String::from("-") {
			use_stdin = true;
			if n == 1 {
				quit_on_stdin = true;
			}
		} else {
			let mut new_file:File = match File::open(&arguments[n]) {
				Err(why) => {
					eprintln!("nand: could not open {}: {}",&arguments[n],why);
					exit(1);
				},
				Ok(file) => file,
			};
			files.push(new_file);
		}
	}
	loop {
		let mut output_buffer:[u8;1] = [0xFF];
		for n in 0..files.len() {
			let mut file_buffer:[u8;1] = [0xFF];
			match files[n].read(&mut file_buffer) {
				Err(_why) => {
					exit(1);
				},
				Ok(0) => if n == 0 && !quit_on_stdin {
					exit(0);
				},
				Ok(_) => (),
			};
			output_buffer[0] = !(output_buffer[0] & file_buffer[0]);
		}
		if use_stdin {
			let mut stdin_buffer:[u8;1] = [0xFF];
			match stdin().read(&mut stdin_buffer) {
				Err(_why) => {
					exit(1);
				},
				Ok(0) => if quit_on_stdin {
					exit(0);
				},
				Ok(_) => (),
			};
			output_buffer[0] = !(output_buffer[0] & stdin_buffer[0]);
		}
		match stdout().write_all(&output_buffer) {
			Err(_why) => {
				exit(0);
			},
			Ok(_) => (),
		};
	}
}
