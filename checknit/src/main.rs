#[no_mangle]
static EMBEDDED_COPYRIGHT: &str = r#"checknit, an init script + simple UI for checkplz

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
along with checknit.  If not, see <http://www.gnu.org/licenses/>."#;

mod init;
mod integrity;
mod ui;

use std::{panic, thread::sleep, time::Duration};
use ui::power::power;

pub(crate) fn press_the_any_key() {
	println!("Press any key to continue...");
	use std::io::{stdin, stdout, Write};
	use termion::{input::TermRead, raw::IntoRawMode};
	let mut stdout = stdout().into_raw_mode().unwrap();
	stdout.flush().unwrap();
	stdin().events().next();
}

#[cfg(feature = "backtrace")]
fn panic_hook(info: &panic::PanicInfo) {
	use backtrace::Backtrace;

	let bt = Backtrace::new();
	eprintln!(
		"{}\n{:?}\n\nAutomatically shutting down in 30 seconds!",
		info, bt
	);
	unsafe {
		libc::sync();
		sleep(Duration::from_secs(30));
		libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF);
		std::process::exit(0);
	}
}

#[cfg(not(feature = "backtrace"))]
fn panic_hook(info: &panic::PanicInfo) {
	eprintln!("{}\n\nAutomatically shutting down in 30 seconds!", info);
	unsafe {
		libc::sync();
		sleep(Duration::from_secs(30));
		libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF);
		std::process::exit(0);
	}
}

fn main() {
	panic::set_hook(Box::new(panic_hook));

	std::env::set_var("TERM", "linux");
	std::env::set_var("TERMINFO", "/etc/terminfo");

	if unsafe { libc::fork() } == 0 {
		sleep(Duration::from_millis(100));
		integrity::verify_integrity();
		loop {
			ui::ui();
		}
	} else {
		init::init();
	}
}
