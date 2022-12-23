fn main() {
    // declaring variable. Notice rust automatically assign i32 type
    let x = 5;
    println!("x: {x}");
    // x = 6; WON'T WORK because x inmutable

    let mut y = 6;
    println!("y: {y}");
    y = 7; // this works since y mutable
    println!("y: {y}");

    let z: i8 = 4;
    println!("z: {z}");
    { // shadowing in new scope
        let z: i8 = 8;
        println!("z inside inner scope: {z}");
    }
    println!("z outside: {z}");

    // constants
    const ANSWER:i32 = 21 * 2; // type is required for constant!
    println!("answer: {ANSWER}");

    // shadowing rule: immutable of different types can use same name!
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {spaces}"); // this references latest assignment (?)
}
