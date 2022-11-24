fn main() {
    hello_world();
    variables();
    data_types();

    println!("The addition of 10 and 20 is {}\n", add(10, 20));

    _loop();
    while_loop();
    for_loop();

    conditionals();
}

fn hello_world() {
    println!("\n\t\t\t***Basics of Rust***\t\t\t\n")
}

fn variables() {
    /* VARIABLES */
    /* are immutable by default */
    /* Thus, the value cannot be changes */
    /* To change, they must be marked mutable */
    /* Using the "mut" keyword */
    /* But the new value must be of same datatype */

    let x: i32 = 5;
    println!("The value of x is {}", x);

    /* SHADOWING */
    /* Variables can be redeclared with same or diff datatype */
    /* This concept is know as shadowing */
    /* Once the variable is shadowed, it's no longer rquired to be marked as mutable */
    /* Marking it mut will lead to compile time error */

    let x: &str = "10";
    println!("The value of x is {}\n", x);

    /* CONSTANTS */
    /* Contants are declared using const keyword and are different from immutable variables */
    /* The datatype must be mentioned */
    /* Cannot be shadowed */
    /* Must be assigned a constant expression */
    /* Instead of any function */

    const AGE: i32 = 20;
    println!("My age is {}\n", AGE);
}

fn data_types() {
    /* SCALER */
    /* 1. int [i8, i16, i32 (default), i64, i128] and [u8, u16, u32, u64, u128] */
    /* 2. Float [f32, f64] */
    /* 3. bool */
    /* 4. char */

    /* COMPOUND */

    /* TUPLES */
    /* Fixed length lists */

    let error_codes = (200, 404, "500");
    let (ok, _, server_error) = error_codes;
    let not_found = error_codes.1;

    println!("HTTP code for OK is {}", ok);
    println!("HTTP code for server error is {}", server_error);
    println!("HTTP code for not found is {}\n", not_found);

    /* ARRAYS */
    /* Homogeneous in nature */
    /* Fixed lengths */

    let languages = ["Rust", "Python", "JavaScript"];
    let bytes = [0; 8];

    println!("I'm learning: {}", languages[0]);
    println!("The bytes array has {} elements\n", bytes.len());
}

fn add(num1: i32, num2: i32) -> i32 {
    let sum = num1 + num2;

    sum

    /* The return expression can be returned without the return statement */
    /* Just mention the expression at the end without semicolon */
}

fn _loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter;
        }

        println!("The counter is {}", counter);
    };

    println!("\nThe loop returned {}\n", result);
}

fn while_loop() {
    let mut counter = 3;

    while counter > 0 {
        println!("{counter}...");

        counter -= 1;
    }

    println!("GO!!!\n");
}

fn for_loop() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("The element is {}", element);
    }

    println!();

    for i in 0..5 {
        println!("arr[{}] = {}", i, arr[i]);
    }

    println!();
}

fn conditionals() {
    const NUMBER: i32 = 20;

    if NUMBER % 2 == 0 {
        println!("The {} is even", NUMBER);
    } else if NUMBER % 2 == 1 {
        println!("The {} is odd", NUMBER);
    } else {
        println!("It's a floating point number")
    }

    println!();
}
