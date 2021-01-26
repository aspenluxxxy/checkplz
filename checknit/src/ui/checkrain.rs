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

use std::{process::Command, thread::sleep, time::Duration};

pub fn checkra1n() {
	Command::new("/bin/checkra1n")
		.spawn()
		.expect("Failed to run checkra1n")
		.wait()
		.expect("Failed to run checkra1n");
	println!("\n\nIf this is your first time jailbreaking using checkra1n,\n it is highly reccomended you bootstrap with odysseyra1n afterwards,\n to get the modern Procursus bootstrap and Sileo package manager.\n");
	// Ensure user actually reads this rather than spamming spacebar
	sleep(Duration::from_secs(5));
	crate::press_the_any_key();
}
