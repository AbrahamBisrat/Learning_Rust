// structs with tuples
#![allow(dead_code)]


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // a method to check if self rec can contain another rect
    fn can_hold(&self, other: &Rectangle) -> bool {
        // check if width is same or more and same for height
        self.width > other.width && self.height > other.height
    }

    // associated functions - static methods from Java
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    
    // associate constructor
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels", area(width1, height1));
    

    // introducing tuples
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels", area_tuple(rect1));
    

    // more refactoring with Rectangle struct to add more meaning to it, instead of using index:
    // naming
    
    let rect2 = Rectangle { width: 30, height: 50, };
    println!("The are of the rectangle is {} square pixels", area_tp_struct(&rect2));
    
    println!("\nThe rectangle data {:#?}", &rect2);
    
    // refactored further to use it's own method
    println!("\nThe area of the rectangle is {} square pixels", rect2.area());
    
    // can hold?
    let rect_third = Rectangle { width: 30, height: 50, };
    println!("Does 3 fit on 2 {}", rect2.can_hold(&rect_third));

    let rect_fourth = Rectangle { width: 20 , height: 40, };
    println!("Does 4 fit on 2 {}", rect2.can_hold(&rect_fourth));
    
    // associated methods
    let square_rect = Rectangle::square(20);
    println!("Square formed from Rect struct {:#?}", &square_rect);

    // associate new
    let rect_new = Rectangle::new(70, 100);
    println!("Rect from associated method {:#?}", rect_new);
}

fn area_tp_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
