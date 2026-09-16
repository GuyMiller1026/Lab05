use std::io;

fn main() {
    println!("{}", is_even(4));

    println!("{}", is_prime(5));
}

// C++ == bool is_even(int number)
// Python == def is_even(number)
fn is_even(number: i32) -> bool {
	if number % 2 == 0 {
                // Use the return keyword -- needs ;
		// return true;
                true // Put the return value or variable, no ;
	} else {
		return false;
	}
}

fn is_prime(number: i32) -> bool {
	if number % 2 != 0 {
		true
	} else {
		return false;
	}
}
