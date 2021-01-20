use std::process::{exit, Command};

pub fn shell() {
	println!("You're on your own now, have fun :)");
	Command::new("/bin/busybox")
		.arg("ash")
		.spawn()
		.expect("Failed to launch ash? wtf")
		.wait()
		.expect("Failed to launch ash? wtf");
	crate::power(libc::LINUX_REBOOT_CMD_POWER_OFF);
	exit(0);
}
