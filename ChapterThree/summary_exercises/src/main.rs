// Exercise from the book
// Conversion from Celcius to Farenheight and vice versa

use std::io;

fn main() {
    loop {
        println!("Simple tool for Tempreture units conversion");
        println!("1. Celcius to Farenheight");
        println!("2. Farenheight to Celcius");
        println!("choose 1 or 2 to continue any other to exit.");

        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let selection: i32 = match selection.trim().parse() {
            Ok(choice) => choice,
            _ => 3 as i32
        };

	// input validation and error checking
	if &selection < &1 || &selection > &2 {
		println!("program terminated by user input");
                break;
         };
	
        // Take user input
	println!("\n Enter the tempreture"); // check selection and make it more meaningful - enter celcius value?
        let mut user_temp_input = String::new();

        io::stdin()
            .read_line(&mut user_temp_input)
            .expect("Failed to read user input");
        let user_temp_input : f32 = match user_temp_input.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("something went wrong");
                continue;
            }
        };

        if selection == 1 {
            // celcius to farenheight
            let converted_farenheight: f32 = celcius_to_farenheight(&user_temp_input);
            println!("{} converted to Farenheight is {} \n", 
			&user_temp_input, &converted_farenheight);
        } else if selection == 2 {
            let converted_celcius: f32 = farenheight_to_celcius(&user_temp_input);
            println!("{} converted to Celcius is {} \n",
			&user_temp_input, &converted_celcius);
        }
    }
    //println!("Exited successfully!");
}

fn celcius_to_farenheight(celcius: & f32) -> f32 {
 	(celcius * (9 as f32 / 5 as f32)) + 32.
}
fn farenheight_to_celcius(far: & f32) -> f32 {
   	 (5 as f32 / 9 as f32) * (far - 32.)
}










