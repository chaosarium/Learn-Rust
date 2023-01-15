fn main() {

    // Without struct, we would be doing this
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // With struct, we can define a rectangle and implement a method to calculate its area
    let rectangle2 = Rectangle {
        width: 500,
        height: 40,
    };

    // what about printing the rectangle? 
    println!("rectangle2 {:?}", rectangle2); // {:?} is pretty print. It lets us print using 
    println!("rectangle2 {:#?}", rectangle2); // {:#?} is similar but does it in multiple lines
    dbg!(&rectangle2); // this provides some debug info like line number

    // a way to print area by accessing fields directly
    println!("The area of the rectangle2 is {} square pixels.",
        rectangle2.width * rectangle2.height
    );

    // but we can also get result from their methods.
    println!("The area of the rectangle2 is {} square pixels.", rectangle2.area());
    // or..
    println!("The area of the rectangle2 is {} square pixels.", Rectangle::area(&rectangle2));
    println!("The circumference of the rectangle2 is {} square pixels.", rectangle2.circumference());

    // well, we can implement std::fmt::Display for the rectangle to format how it gets printed
    
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implements rectangle area calculation
impl Rectangle {
    fn area(&self) -> u32 { // &self is short for `self: &Self!`, in which Self refers to Rectangle
        self.width * self.height
    }
    
    fn circumference(self: &Self) -> u32 {
        2 * self.width + 2 * self.height
    }
}