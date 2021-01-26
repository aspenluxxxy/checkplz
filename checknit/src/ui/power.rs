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

pub use libc::{LINUX_REBOOT_CMD_POWER_OFF, LINUX_REBOOT_CMD_RESTART};
use std::process::exit;

pub fn power(how_to: libc::c_int) {
	unsafe {
		libc::sync();
		libc::reboot(how_to);
	}
	exit(0);
}
