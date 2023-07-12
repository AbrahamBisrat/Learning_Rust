use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Enter your guess");
 
    let secret_num = rand::thread_rng().gen_range(1, 101);
    //println!("The random generated is {}: ", secret_num);

    //let mut guess = String::new();

    loop {
	let mut guess = String::new();
        io::stdin().read_line(&mut guess)
	    .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => {
		println!("Wrong input, non-negative integers only. Eg. 1, 2, 3");
		continue;
	    },
	};

        println!("You guessed: {}", guess);
        //println!("The secret number is {}", secret_num);
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!"),
	    Ordering::Greater => println!("Too Big!"),
	    Ordering::Equal => {
		println!("Congrats You Won!");
		break;
	    }
        }
    }
}
