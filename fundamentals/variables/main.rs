fn main() {

	/* All variables that are defined are IMMUTABLE by default.	
	mut qualifier makes variable mutable. */
	let mut x = 45;

	println!("The value of x is {}", x);

	x = 60;

	println!("The value of x is {}", x);

	// Variable data types. Use u64 unsigned 64 bit int if not negative.
	let y: u64 = 45; // u64

	let f: f32 = 6.7; // f32

	let b: bool = false;
}