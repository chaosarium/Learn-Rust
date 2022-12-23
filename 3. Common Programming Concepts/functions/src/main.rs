fn main() {
    another_function();
    func_with_param(32);
    let x = func_with_return(32);
    println!("got {x} as output");
    let x = func_with_return2(32);
    println!("got {x} as output");
}

fn another_function() {
    println!("Hello, world again");
}

fn func_with_param(x: i32) { // input type annotation!
    println!("input is {x}");
}

fn func_with_return(x: i32) -> i32 { // return type annotation!
    x / 2 // this is what it returns!
    /* 
    Note that rust `x / 2` is an expression that evaluates to something. It is different from a statement that does something.  
    */
}

fn func_with_return2(x: i32) -> i32 { // return type annotation!
    2 + {x / 2}
    /* 
    We can put expression in {} too! 
    */
}