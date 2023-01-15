# Enums

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

enum Message {
    Quit, // no associated data
    Move { x: i32, y: i32 }, // like a struct
    Write(String),
    ChangeColor(i32, i32, i32), // three i32
}
/* which groups these together
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/
```

Rust does not have NULL, this enum helps to let a variable have multiple states. `Option` is already included in the standard lib
```rust
enum Option<T> { // the <T> indicates a generic type
    None,
    Some(T),
}
```
And then:
```rust
let some_number = Some(5);
let some_char = Some('e');
assert_eq!(some_number.is_some(), true); // also from standard lib

let absent_number: Option<i32> = None;

let x = Some("air");
assert_eq!(x.unwrap(), "air"); // this is how we get the value
```

