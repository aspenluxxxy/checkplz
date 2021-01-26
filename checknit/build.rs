use std::{env, fmt::Write, fs, path::Path};

const DIRS_TO_SCAN_REL: &[(&str, &str, &[&str])] = &[
	(
		"../build/sysroot/bin",
		"../build/sysroot",
		&[
			"busybox",
			"checkra1n",
			"dropbearmulti",
			"iproxy",
			"irecovery",
			"scp",
			"usbmuxd",
		],
	),
	("../config/odysseyra1n", "../config", &[]),
];

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("checksum.rs");
	let mut output = String::new();

	writeln!(output, "const CHECKSUMS: &[(&str, [u8; 32])] = &[").unwrap();

	for (dir, prefix, whitelist) in DIRS_TO_SCAN_REL {
		let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
		for file in fs::read_dir(dir).unwrap() {
			let file = file.unwrap();
			if !file.path().exists()
				|| (!whitelist.is_empty()
					&& !whitelist.contains(&file.file_name().to_str().unwrap()))
			{
				continue;
			}
			let path_entry = [
				"/",
				file.path()
					.strip_prefix(env!("CARGO_MANIFEST_DIR"))
					.unwrap()
					.strip_prefix(prefix)
					.unwrap()
					.display()
					.to_string()
					.as_str(),
			]
			.join("");

			writeln!(
				output,
				"\t(\"{}\", {:?}),",
				path_entry,
				blake3::hash(&fs::read(file.path()).unwrap()).as_bytes()
			)
			.unwrap();
		}
	}

	writeln!(output, "];").unwrap();

	fs::write(&dest_path, output).unwrap();

	println!("cargo:rerun-if-changed=build.rs");
}
