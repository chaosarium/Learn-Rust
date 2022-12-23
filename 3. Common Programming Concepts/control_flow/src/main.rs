fn main() {
    // if statemetn
    let x = -3;
    if x > 2 {
        println!("number greater than 2");
    } else if x != 2 {
        println!("number less than 2");
    } else {
        println!("number is 2");
    }
    
    // inline condition
    let y = if 1+1==3 {3} else {4};
    println!("y evaluated to {y}");

    // loop just loops until break
    loop {
        println!("again!");
        break;
    }

    // evaluating expression from loop --- put something after break
    let mut counter = 0;
    println!("{}", loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    });

    // labelling loop helps you to break the right loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut count = 0;
    while count != 111 {
        count += 1;
    }
    println!("final count is {count}");

    // for loop on array
    let a = [13, 23, 33, 43, 53];
    for element in a {
        println!("the value is: {element}");
    }

    // looping through numbers?
    for number in (1..4) { // (a..b) makes a range!
        println!("{number}!");
    }
    for number in (4..=6).rev() { // and we can reverse it // =6 means include 6
        println!("{number}!");
    }
}
