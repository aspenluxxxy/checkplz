/*
	checknit, an init script + simple UI for checkplz

	Copyright (C) 2021  aspen

	This file is part of checknit.

	checknit is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 2 of the License, or
	(at your option) any later version.

	checknit is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with checknit.  If not, see <http://www.gnu.org/licenses/>.
*/

use blake3::Hasher;
use cursive::views::{CircularFocus, Dialog, TextView};
use std::{
	fs::File,
	io::{BufReader, Read},
};

include!(concat!(env!("OUT_DIR"), "/checksum.rs"));

pub fn verify_integrity() {
	let mut failed: Vec<&'static str> = vec![];
	let mut buffer = vec![0u8; 8192];
	let mut hasher = Hasher::new();
	for (file, hash) in CHECKSUMS.iter() {
		println!("[checknit] verifying {}", file);
		let mut reader = match File::open(file) {
			Ok(f) => BufReader::new(f),
			Err(_) => continue,
		};
		while let Ok(len) = reader.read(&mut buffer) {
			if len < 1 {
				break;
			}
			hasher.update(&buffer[..len]);
		}
		let calculated_hash = hasher.finalize();
		hasher.reset();
		if calculated_hash != *hash {
			failed.push(file);
		}
	}
	if !failed.is_empty() {
		let mut siv = cursive::default();
		siv.add_layer(CircularFocus::wrap_tab(
			Dialog::around(TextView::new(failed.join("\n")))
				.title("Corrupted Files!")
				.button("Shutdown", |_| {
					crate::power(libc::LINUX_REBOOT_CMD_POWER_OFF)
				})
				.button("Continue", |s| s.quit()),
		));
		siv.run();
	}
}
