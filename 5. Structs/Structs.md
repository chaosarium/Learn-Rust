Struct definition

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Struct instance creation and access
```rust
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // shorthand for creating new struct instance based on another
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

A struct builder
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username, // shorthand for "username: username"
        active: true,
        sign_in_count: 1,
    }
}
```

## Methods in a struct
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

! Rust has automatic referencing and dereferencing... there's no equivalence to C's `->`.

Associated function are those that don't need an instance of the struct to work with. Think of this as a general function about the category. This code implements `Rectangle::square(3);`, for example.
```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```