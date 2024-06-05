//module_path!(`src/bin/datatypes.rs`);

pub(crate) fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // adding underscore allows for easier reading for large numbers
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // shadowing: allows to change the type of a variable
    x = 11;
    println!("The value of x is: {}", x);
    x = 12;
    println!("The value of x is: {}", x);

    // run our function
    println!("The sum is: {}", my_function(5, 55));

    let number1 = 5;

    // if statements:
    if number1 < 10 {
        println!("condition was true");
    } else if number1 > 3{
        println!("condition was true too");
    } else {
        println!("condition was false");
    }

    // if statements as expressions:
    let condition = true;
    let number2 = if condition { 5 } else { 6 };
    println!("the number would be 5 since condition is true {}", number2);

    // loops: wow this one is sorta like while loops in python
    let mut counter = 0;
    loop{
        println!("again!");
        counter += 1;
        // set a condition to break out of the loop
        if counter == 10 {
            break;
        }        
    }
    // If you want to return a value from a loop, you can use a `return` statement instead of a `break` statement. Here's an example:
    counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("Counter: {}", result); // prints "Counter: 10"

    //In this example, the loop runs until `counter` reaches 10, at which point the loop is 
    //broken out of with the value of `counter` (10) being returned and assigned to the `result` variable. 
    //Finally, the value of `result` is printed.

    // for loops:
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // for loops with range: but reverse order
    for number in (1..5).rev() {
        println!("{}!", number);
    }

    // for loops with range:
    for number in 1..4 {
        println!("{}!", number)
    }

    // classic while loop
    let mut number3 = 3;
    while number3 != 0 {
        println!("{}!", number3);
        number3 -= 1;
    }

    // classic while loop with break
    let mut number4 = 3;
    while number4 != 0 {
        println!("{}!", number4);
        number4 -= 1;
        if number4 == 0 {
            break;
        }
    }


    // classic for loop with break
    for number5 in (1..4).rev() {
        println!("Check out number5: {}!", number5);
        if number5 == 1 {
            break;
        }
    }

    // classic for loop with continue
    for number6 in 1..4 {
        if number6 == 2 {
            continue;
        }
        println!("{}!", number6);
    }

    // classic while loop with continue
    let mut number7 = 3;
    while number7 != 0 {
        if number7 == 2 {
            number7 -= 1;
            continue;
        }
        println!("{}!", number7);
        number7 -= 1;
    }

    // stack and heap
    // stack: last in first out
    // heap: random access

    // Ownership rules:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    let x = 5;
    let y = x;
    // for integers, the value is copied
    // but this changes for strings
    // this is only valid for datatypes: integers, floats, booleans, characters
    println!("x is: {}", x);
    println!("y is: {}", y);
    // statement and expression:
    // statement: does not return a value
    // expression: returns a value
    println!("================================================================");
    println!("======================== expression vs statement ===============");
    println!("================================================================");
    // this is a statement:
    let mut x = 9;
    println!("x is: {}", x);
    // this is a statement:
    //let z = (x = 6);
    let z = x = 6;
    println!("z is: {:?}", z);

    // this is an expression:
    let y = {
        let x = 3;
        x + 1
    };
    println!("y is: {}", y);
    println!("x is: {}", x);

    // String type onweship demo:
    let s1 = String::from("hello");
    //let s2 = s1;
    let s2 = s1.clone(); // this is a deep copy!
    // for strings, the value is moved!
    println!("s2 is: {}", s2);
    println!("s1 is: {}", s1);
    
    // rust uses a borrow checker to make sure that references are valid
    // this is called the "borrow checker"
    // "borrow checker" is a compile time check
    // it is a way to prevent dangling references
    // rust does have a reference counter, but it is not used for memory management
    
    // this is a function that takes ownership of a string
    let s3 = String::from("hello");
    takes_ownership(s3);

    // this is a function that takes a reference to a string
    let s3 = String::from("hello");
    let len = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, len);




    
    //Dangling references are a particular type of memory safety problem that can occur in languages with pointers.
    //Dangling references occur when a pointer references a location in memory that may have been given to someone else,
    //by freeing some memory while preserving a pointer to that memory.
    //In other words, they’re pointers that were pointing to memory that may have been deallocated.
    //Dangling references become a problem because they’re invalid pointers that may still be in use by your code.
    //Rust prevents this problem by never letting you create a dangling reference.
    // this is a function that takes a reference to a string
    //let s3 = String::from("hello");
    //let len = calculate_length(&s3);
    
    // this is a dangling reference
    // this is a compile time error
    //let s4 = String::from("hello");



}

fn my_function(x: i32, y:i32) -> i32 {
    println!("=================================");
    println!("first number is {} and second number is {}", x, y);
    println!("=================================");
    x + y
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}

// this is a function that returns a string to the caller to take ownership of it back again (move)
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

// this is a function that takes a reference to a string
