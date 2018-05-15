extern crate cc;

/// Adds a dependency to the build script.
fn add_dependency(dep: &str) { println!("cargo:rerun-if-changed={}", dep); }

fn main() {
    use std::{env,process::Command};

    // Compile the lexer .
    add_dependency("src/poc.l");
    let bin = env::var("LEX").unwrap_or(String::from("flex"));
	Command::new(bin)
		.arg("-opoc.c")
		.arg("src/poc.l")
        .status()
        .expect("failed to execute Flex's process");

	cc::Build::new()
        	.file("poc.c")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-variable")
            .flag_if_supported("-Wno-unused-function")
	        .compile("poc.a");
}
