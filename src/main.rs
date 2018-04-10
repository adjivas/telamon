extern crate poc;

fn main() {
    unsafe {
	println!("lines: {}", poc::lines);
    }
}
