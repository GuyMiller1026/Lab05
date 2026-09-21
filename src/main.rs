use std::io;

fn main() {
    println!("{}", is_even(4));

    println!("{}", is_prime(5));
    
    println!("{}", digit_sum(12));
    
    println!("{}", count_divisors(6));
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
fn digit_sum(mut num: i32) -> i32 {
	let mut sum = 0;
	while num > 0 {
		sum += (num % 10);
		num /= 10;
	}
		
	return sum
}

fn count_divisors(num: i32) -> i32 {
	let mut div = 0;
	for i in 1..=num {
		if num % i == 0{
			div += 1;
		}
	}	
	return div;
	
}
