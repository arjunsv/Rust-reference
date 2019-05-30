fn main() {
	let mut n = 0;

	// Regular loop.
	loop {
		n += 1;

		if n == 7 {
			continue;
		}

		if n > 10 {
			break;
		}

		println!("The value of n is {}", n);
	}

	// While loop.
	while n <= 50 {
		if n % 5 == 0 {
			println!("n is {}", n);
		}

		n += 1;
	}

	let numbers = 3..52;

	// For loop.
	for i in numbers {
		println!("The number is {}", i);
	}

	let animals = vec!["Rabbit", "Dog", "Cat"];

	for (index, a) in animals.iter().enumerate() {
		println!("The index is {} The animal is {}", index, a)
	}
}