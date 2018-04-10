use std::process::Command;

fn main() {
	Command::new("make")
		.arg("-C")
		.arg(env!("CARGO_MANIFEST_DIR"))
		.arg("static")
		.arg("clean")
                .status()
                .expect("failed to execute process");
}
