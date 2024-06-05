/*
Data types: scalar data types:
    - integers: i8, i16, i32, i64, i128, isize(signed integers), arch (system size-32 or 64) 
        interger overflow: when the value of a variable exceeds the maximum value of its type
        u8: 0 to 255
        i8: -128 to 127
        u16: 0 to 65,535
        i16: -32,768 to 32,767
        u32: 0 to 4,294,967,295
        i32: -2,147,483,648 to 2,147,483,647
        u64: 0 to 18,446,744,073,709,551,615
        i64: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
        u128: 0 to 3.402823669209385e+38
        i128: -1.7014118346046923e+38 to 1.7014118346046923e+38
        isize: -2^(n-1) to 2^(n-1)-1
        usize: 0 to 2^n-1

    - floating point numbers: f32, f64
    - booleans: true, false
    - characters: 'a', 'b', 'c', '1', '2', '3', '!', '?', ' ', '\t', '\n'

    compound data types:
    - tuples: fixed length, can contain multiple types, can be accessed by index, can be destructured, can be used as function arguments, 
    can be used as return values, can be used as array elements, can be used as array elements, can be used as array elements, 

    - arrays: fixed length, can contain multiple types, can be accessed by index, can be used as function arguments, 

 */

fn main(){
    // integers:
    let a_deci_int = 98_222; // decimals
    println!("The value of a decimal Int is: {}", a_deci_int);

    let b_hex = 0xff; // hex
    println!("The value of a hex Int is: {}", b_hex);

    let c_octal = 0o77; // octal
    println!("The value of a octal Int is: {}", c_octal);

    let d_bin = 0b1111_0000; // binary
    println!("The value of a binary Int is: {}", d_bin);

    let e = b'A'; // byte (u8 only)
    println!("The value of a byte Int is: {}", e);

    // floating point numbers:
    let f = 2.0; // f64
    let g: f32 = 3.0; // f32
    println!("The value of our floats are: {}, {}", f, g);
    
    // integer overflow:
    let h_intover: u8 = 255; // 256 will become 0, integer overflow
    println!("The value of our integer overflow is: {}", h_intover);

    // addition:
    let sum = 5 + 10;
    println!("sum is {}", sum);

    // subtraction:
    let difference = 95.5 - 4.3;
    println!("difference is {}", difference);

    // multiplication:
    let product = 4 * 30;
    println!("product is {}", product);
    // division:
    let quotient = 56.7 / 32.2;
    println!("quotient is {}", quotient);

    // remainder:
    let remainder = 43 % 5;
    println!("remainder is {}", remainder);

    // booleans:
    let t_bool = true;
    let f_bool: bool = false; // with explicit type annotation
    println!("The values of all our booleans are: {}, {}", t_bool, f_bool);

    // characters:
    let c_char = 'z'; // char
    let z_char = 'ℤ'; // unicode
    let heart_eyed_cat_char = '😻'; // char is 4 bytes in size
    
    println!("The values of all our char are: {}, {}, {}", c_char, z_char, heart_eyed_cat_char);

    // tuples:
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y); 
    println!("The value of z is: {}", z);
    println!("The value of x is: {}", x);
    let five_hundred = tup.0; // accessing by index
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // rust datatypes: arrays and vectors (dynamic arrays)  

    // arrays:
    let a = [1, 2, 3, 4, 5]; // array
    let first_arr_ele = a[0]; // accessing by index
    let second_arr_ele = a[1];
    println!("The values of 0 and 1 elements of our array is: {}, {}", first_arr_ele, second_arr_ele);

    // vector:
    let v = vec![1, 2, 3, 4, 5]; // vector
    let first_vec_ele = v[0]; // accessing by index
    let second_vec_ele = v[1];
    println!("The values of 0 and 1 elements of our vector is: {}, {}", first_vec_ele, second_vec_ele);

}