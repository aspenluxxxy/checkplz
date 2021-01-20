/*
	Based off of https://github.com/coolstar/Odyssey-bootstrap/blob/master/procursus-deploy-linux-macos.sh
*/

use std::{
	process::{Command, Stdio},
	thread::sleep,
	time::Duration,
};

const ODYSSEYRAIN_BOOTSTRAP_1600: &str = "/etc/odysseyra1n/bootstrap_1600.tar";
const ODYSSEYRAIN_DEPLOY: &str = "/etc/odysseyra1n/odyssey-device-deploy.sh";
const ODYSSEYRAIN_MIGRATION: &str = "/etc/odysseyra1n/migration";
const ODYSSEYRAIN_SILEO_DEB: &str = "/etc/odysseyra1n/org.coolstar.sileo_2.0.0b6_iphoneos-arm.deb";

pub fn odysseyra1n() {
	let mut iproxy = Command::new("/bin/iproxy")
		.args(&["4444", "44"])
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.stdin(Stdio::null())
		.spawn()
		.expect("Failed to launch iproxy!");
	println!(
		"Copying files to device...\nDefault password is `alpine` if you have not set it already!"
	);
	Command::new("/bin/scp")
		.args(&[
			"-P4444",
			ODYSSEYRAIN_BOOTSTRAP_1600,
			ODYSSEYRAIN_DEPLOY,
			ODYSSEYRAIN_MIGRATION,
			ODYSSEYRAIN_SILEO_DEB,
			"root@127.0.0.1:/var/root/",
		])
		.spawn()
		.expect("Failed to copy files over SSH!")
		.wait()
		.expect("Failed to copy files over SSH!");
	Command::new("/bin/dbclient")
		.args(&[
			"root@127.0.0.1/4444",
			"zsh /var/root/odyssey-device-deploy.sh",
		])
		.spawn()
		.expect("Failed to SSH into device!")
		.wait()
		.expect("Failed to SSH into device!");
	println!("\nOdysseyra1n / Procursus *should* be bootstrapped now, barring any weird errors!");
	iproxy.kill().expect("Failed to terminate iproxy");

	println!("\nCredit to coolstar and the Sileo, Electra, Chimera, and Odyssey teams\n for their work on Sileo and odysseyra1n,\n and to Diatrus and the Procursus team for the Procursus bootstrap!\n");
	sleep(Duration::from_secs(2));
	crate::press_the_any_key();
}
