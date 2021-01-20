/*
	checknit, an init script + simple UI for  checkplz

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
use hex_literal::hex;
use std::{
	fs::File,
	io::{BufReader, Read},
};

const CHECKSUMS: &[(&str, [u8; 32])] = &[
	(
		"/bin/checkra1n",
		hex!("12c179aa4b1454072006d9370312e5773c2c5ad196d9b1356835020e587c7072"),
	),
	(
		"/etc/odysseyra1n/bootstrap_1600.tar",
		hex!("198413b8829061cc54616b58be7e6df33e0f8aca0b8b3818dff3e6ef552de608"),
	),
	(
		"/etc/odysseyra1n/migration",
		hex!("004acb8c6d1172b83bb1a4dd8ed3bc0c2ea537d0da63e0ba7bb437b9ed7d7d51"),
	),
	(
		"/etc/odysseyra1n/odyssey-device-deploy.sh",
		hex!("b35db81dbc7f6b75029f1e6a1251eedb8ceb2bf270ca7b437668776325f041fd"),
	),
	(
		"/etc/odysseyra1n/org.coolstar.sileo_2.0.0b6_iphoneos-arm.deb",
		hex!("f1230bbb8866a1bb03e49acc3c1c603a307c0e20bba1c564936bbe1ef8b7f9df"),
	),
	(
		"/etc/odysseyra1n/org.swift.libswift_5.0-electra2_iphoneos-arm.deb",
		hex!("589fed35619b198d643196bcea84ee6b1d57d760efc88cdeab39d6fbd15800da"),
	),
];

pub fn verify_integrity() {
	let mut failed: Vec<&'static str> = vec![];
	let mut buffer = vec![0u8; 8192];
	let mut hasher = Hasher::new();
	for (file, hash) in CHECKSUMS.iter() {
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
