// take a string and return the first word from the string.

fn main() {
    let sample_string = "sample string";
    let another_string = "another set of strings"; 
    let one_big_string = "jsalkjf;lasjdflkjasdlkfja;sldkjf;laksj";

    println!("Output : {} ", first_string(sample_string));
    println!("Output : {} ", first_string(another_string));
    println!("Output : {} ", first_string(one_big_string));
    
    
    println!("Output : {} ", first_string_rusty(sample_string));
    println!("Output : {} ", first_string_rusty(another_string));
    println!("Output : {} ", first_string_rusty(one_big_string));
}
fn first_string(input: &str) -> &str{
    for (i, each) in input.chars().enumerate() {
        if each == ' ' { return &input[..i]; }
    }
    &input[..]
}

// more rusty way of doing it, example from the book

fn first_string_rusty(input: &str) -> usize {
    let bytes = input.as_bytes();

    for (i, &each_char) in bytes.iter().enumerate() {
        if each_char == b' ' { return i; }
    }
    input.len()
}
