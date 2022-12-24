## Ownership

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