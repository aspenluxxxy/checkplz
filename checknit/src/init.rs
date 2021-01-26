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

use std::process::Command;

pub fn init() {
	Command::new("/bin/busybox")
		.arg("--install")
		.arg("/bin")
		.spawn()
		.expect("Failed to set busybox symlinks")
		.wait()
		.expect("Failed to set busybox symlinks");
	Command::new("/bin/busybox")
		.arg("mount")
		.arg("-t")
		.arg("devtmpfs")
		.arg("none")
		.arg("/dev")
		.spawn()
		.expect("failed to mount devtmpfs")
		.wait()
		.expect("failed to mount devtmpfs");
	Command::new("/bin/busybox")
		.arg("mount")
		.arg("-t")
		.arg("proc")
		.arg("p")
		.arg("/proc")
		.spawn()
		.expect("failed to mount proc")
		.wait()
		.expect("failed to mount proc");
	Command::new("/bin/busybox")
		.arg("mount")
		.arg("-t")
		.arg("sysfs")
		.arg("none")
		.arg("/sys")
		.spawn()
		.expect("failed to mount sysfs")
		.wait()
		.expect("failed to mount sysfs");
	Command::new("/bin/busybox")
		.arg("mount")
		.arg("-t")
		.arg("tmpfs")
		.arg("tmpfs")
		.arg("/tmp")
		.spawn()
		.expect("failed to mount tmpfs")
		.wait()
		.expect("failed to mount tmpfs");
	Command::new("/bin/busybox")
		.arg("mdev")
		.arg("-s")
		.spawn()
		.expect("failed to launch mdev -s");
	Command::new("/bin/busybox")
		.arg("mdev")
		.arg("-d")
		.spawn()
		.expect("failed to launch mdev -d");
	Command::new("/bin/busybox")
		.arg("ifup")
		.arg("-a")
		.spawn()
		.expect("failed to set up interfaces");
	Command::new("/bin/usbmuxd")
		.spawn()
		.expect("failed to start usbmuxd");
	std::os::unix::fs::symlink("/proc/self/fd/0", "/dev/stdin")
		.expect("failed to symlink /proc/self/fd/0 to /dev/stdin");
	std::os::unix::fs::symlink("/proc/self/fd/1", "/dev/stdout")
		.expect("failed to symlink /proc/self/fd/1 to /dev/stdout");
	std::os::unix::fs::symlink("/proc/self/fd/2", "/dev/stderr")
		.expect("failed to symlink /proc/self/fd/2 to /dev/stderr");
	std::mem::drop(std::fs::remove_file("/dev/tty"));
	std::os::unix::fs::symlink("/dev/console", "/dev/tty")
		.expect("failed to symlink /proc/console to /dev/tty");
	loop {
		unsafe {
			libc::wait(core::ptr::null_mut());
		}
	}
}
