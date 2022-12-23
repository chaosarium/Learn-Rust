fn main() {
    /* integer types
    - i8    u8 === int8_t, uint8_t
    - i16   u16 === simili
    - i32   u32
    - i64   u64
    - i128  u128
    - isize usize --- depends on architecture. 32 bits -> 32 bits; 64 bits -> 64 bits; etc.
    */
    // type annotation:
    let y: i32 = 4;
    // declaration
    let x = 98_222; // DEC, same as let x = 98222, but easier to read
    let x: u8 = 0xff; // HEX
    let x: u8 = 0x77; // OCT
    let x: u8 = 0b11110000; // BIN
    let x: u8 = b'c'; // BYTE
    println!("{x}");

    // Overflow: rust usually warns about overflow. in release version, overflow uses two's complement

    /* float types
    - f32
    - f64 (default)
    */
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    /* operations
    +, -, *, /, % do what we expesct
    */

    /* boolean type
    - bool
    */
    let t = true;
    let f: bool = false;
    
    /* character type
    - char --- they are 4 bytes! So they can work with unicode
    */
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '🦀';

    /* compound type
    - tuple --- fixed length, groups multiple types together
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // get individual elems
    let last = tup.2; // inde into element
    // empty tuples are called *unit*
    let empty: () = ();
    
    /* array type
    Fixed length!
    Goes in the stack!
    */
    let a = [1, 2, 3, 4, 5]; // defaults to i32
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    // type annotation
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    // default elements
    let a = [0; 3]; // same as let a = [0, 0, 0]
    // array access
    println!("{}", months[2]);
    // index out of bound checked at runtime!!

}
