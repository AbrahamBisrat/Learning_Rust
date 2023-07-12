
const DEVIL_NUMBER: i32 = 666;

fn main() {
    println!("Hello World!");
    another_function();
    devil_number_checker(666);
}

fn another_function() {
    println!("also the others");
}

fn devil_number_checker(suspected_number: i32) {
    println!("{}", suspected_number == DEVIL_NUMBER);
}
