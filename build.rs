extern crate cc;

use std::process::Command;

fn main() {
	Command::new("flex")
		.arg("-opoc.c")
		.arg("src/poc.l")
                .status()
                .expect("failed to execute Flex's process");

	cc::Build::new()
        	.file("poc.c")
	        .compile("poc.a");
}
