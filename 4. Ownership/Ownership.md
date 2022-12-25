# Ownership

- Rust uses an ownership model to manage memory
- No garbage collector
- But also no explicit allocation / freeing

## Memory Segments:

- The stack (fast allocation and access)
- The heap - can be referred to using pointers like in C (slower allocation and access)

## Ownership rules

- Each value has one and only one owner
- Value drooped when owner is out of scope (Rust automatically calls `drop()`)

**Stack only data**

These can get copied. The following code does what you expect:

```rust
let x = 5;
let y = x;
```

**String type as example**

- String literals are hardcoded (read only?)
- Strings contain pointer to a char array allocated on heap. 

To make a String:

```rust
let s = String::from("hello");
```

Desection a string:
- ptr (pointer to char array)
- len (length)
- capacity

Coping heap variables could lead to double free. Instead, Rust moves variable by essentially invalidates the old variable

```rust
let s1 = String::from("hello");
let s2 = s1;
// s1 invalidated. it's "moved" to s2
println!("{}, world!", s1); // does not work
println!("{}, world!", s2); // good
```

See https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html for memory diagram.

We can, however, copy the data in the heap by doing:

```rust
let s2 = s1.clone();
```

## Ownership in functions

```rust
fn main() {
    let s = String::from("hello");
    let x = 5;

    takes_ownership(s); // s moved into function
    // s is no longer valid here
    makes_copy(x); // x copied into function
    // x still valid here
} // s already moved out, so no dropping

fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
} // some_string goes out of scope, so dropped

fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
} // some_integer is on stack, nothing special
```

```rust
fn main() {
    let s1 = gives_ownership(); // moved from function's return
    let s2 = String::from("hello"); // come into scope
    let s3 = takes_and_gives_back(s2); // s2 moved away; 
} // s1 dropped, s2 alway moved away so nothing happens, s3 dropped

fn gives_ownership() -> String {
    let some_string = String::from("yours"); 
    some_string // move to caller, so moves out of scope
} // nothign dropped

fn takes_and_gives_back(a_string: String) -> String {
    a_string // move to caller, so moves out of scope
} // nothign dropped
```

# References and Borrowing

We can do this instead of passing around ownership around. It's kind of like passing pointers around in C.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Instead of being a copy of the address of s1, s is a pointer to s1. We're "referencing" s1, and Rust calls this "borrowing".

We can use `*` to dereference too, just like in C.

- By default, **references are not mutable!** Here's how to pass a mutable reference. 

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- Note that there can be **a maximum of one mutable reference (in the same scope)!** This is to prevent data race.
- Also, **no immutable and mutable of same value that could be used at the same time!**
- **Dangling reference not allowed**, this is to make sure you don't return something that's already freed, etc.

```rust
// bad, does not compile
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // reference to s returned but s dropped
```

# Slice

This allows us to refer to the middle of the bigger chunk of memory.

Making a string slice:

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];

let slice = &s[3..]; // === [3:]
let slice = &s[..]; // === whole thing

```