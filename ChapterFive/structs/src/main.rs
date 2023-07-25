#![allow(dead_code)]
// Structs are an efficient way of storing data
 

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Chapter Five - Structs [ optimized data structures ]");
    
    let user_one = User {
        email: String::from("mail@abisrat.com"),
        username: String::from("boiled_potatos"),
        sign_in_count: 1,
        active: true,
    };

    //println!("User Data {:?}", user_one);
    print_user(&user_one);

    let user_two = User {
        email: String::from("test@test.com"), 
        username: String::from("reconov"), 
        sign_in_count: 1, 
        active: true, 
    };


    print_user(&user_two);

    let user_three = create_user(String::from("fn_user"), String::from("fn_email"), 1, true);

    print_user(&user_three);

    print_user(&create_user(String::from("anon"), String::from("anon@ymous.io"), 1, true));

}

fn print_user(user_struct: &User) {
    println!("User Data {:?}", user_struct);
}

fn create_user(username: String, email: String, sic: u64, active: bool) -> User {
    User {
        username,
        email,
        sign_in_count: sic,
        active,
    }
}
