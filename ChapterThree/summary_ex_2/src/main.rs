// Find the nth fibonnaci number - input from user

use std::io;

fn main() {
	println!("Nth Fibbonacci sequence solver");
	println!("Enter 'n' value");
	
	let mut fib_n = String::new();
	io::stdin().read_line(&mut fib_n).expect("failed to read line");
	let fib_n : u32= match fib_n.trim().parse() {
		Ok(n) => n,
		Err(_err) => panic!("Problem parsing user input")
	};
	println!("The value of {}th Fibbonnacci sequence is : {} \n", &fib_n, fib_of_n(fib_n));
}

fn fib_of_n(n: u32) -> u32 {
	if n == 0 { return 0; }
	else if n == 1 { return 1; }	
	fib_of_n(n - 1) + fib_of_n(n - 2)
}

