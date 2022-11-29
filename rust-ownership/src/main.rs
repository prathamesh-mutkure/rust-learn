/*
    ----------- OWNERSHIP RULES --------------------------------------------
    1. Each value in rust has a variable that's called it's owner
    2. There can only be 1 owner at a time
    3. When the owner goes out of scope, the value will be dropped
    ------------------------------------------------------------------------
*/

fn ownership_in_rust() {
    // code here

    {
        // str is not valid here, not declared yet
        let str: String = String::from("hello world"); // str is valid from this point forward

        // do stuff with str
    } // This scope is now over and str is no longer valid

    // more code
}

fn main() {
    println!("Hello, world!");

    ownership_in_rust();
    copy_and_move();
    references();
    references_rules();
    slices();
    slices_types();

    println!("\n\n")
}

/*************** OWNERSHIP ***************/

fn copy_and_move() {
    println!("\n------ OWNERSHIP ------\n");

    let x = 5;
    let y = x; // copy

    println!("x = {}\ny = {}", x, y);

    /* PRIMITIVE VARIABLES
       - For variables of primitive datatype, rust copies the value
       - Just like C
    */

    let s1 = String::from("Hello World!");
    let s2 = s1; // MOVE (Not shallow copy)

    // let s3 = s2.clone(); // Deep copy (works)

    // ERROR
    // println!("s1 = {}\ns2 = {}", s1, s2)

    // once value of s2 is assigned to s1
    // then, unlike other languages where new variable points to same value, i.e shallow copy
    // In rust, the ownership is moved

    let str = String::from("I'm a string");

    takes_ownership(str);

    // ERROR
    // The ownership of str has been taken away by another function
    // println!("str = {}", str);

    let str1 = gives_ownership();
    let str2 = String::from("Hello");
    let str3 = gives_and_takes_ownership(str2);

    println!("str1 = {}", str1);

    // ERROR
    // println!("str2 = {}", s2);
    // The ownership of s2 has been moved to s2
    // s2 is invalid

    println!("str3 = {}", str3);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}

fn gives_ownership() -> String {
    let a_string = String::from("Hello World");

    a_string
}

fn gives_and_takes_ownership(a_string: String) -> String {
    // Do something with a_string
    // And return it
    // To again transfer ownership to parent function

    a_string
}

/*************** REFERNECES ***************/

fn references() {
    println!("\n------ REFERENCES ------\n");

    // Due to ownership rules,
    // We need to pass ownership and return it
    // To use the variable again in parent
    // This increases code and complexity
    // References solve this
    let s1 = String::from("Hello World");
    let (s2, len1) = get_length_normal(s1);

    println!("The length of `{}` is {}", s2, len1);

    // REFRENCES
    // With references, ownership is not transfered
    // It is rather "borrowed"
    // Also references are immutable by default

    let s3 = String::from("Hello Rust");
    let len_2 = get_length_ref(&s3);
    println!("The length of `{}` is {}", s3, len_2);
}

fn get_length_normal(str: String) -> (String, usize) {
    let length = str.len();

    (str, length)
}

fn get_length_ref(str: &String) -> usize {
    str.len()
}

/*************** REFERNECES RULES ***************/

fn references_rules() {
    println!("\n------ REFERENCES RULES ------\n");

    /*

        - References are immutable by defult
        - To modify them, they must be explicitly marked mutable
        - Mutable references must follow following rules

        TWO RULES OF REFERENCES:

        1. At any given point of time for a particular piece of data in particular scope, you can have either
            - One mutable reference
            - Any number of immutable references

        2. References must always be valid

        - This prevents RACE condition
        - When a variable is being modified and read at the same time
        - Also, prevent memory leaks
    */

    let mut str = String::from("Hello World");

    // Mutable variable with mutable reference
    change_str(&mut str);

    // Multiple immutable references are valid
    let r1 = &str;
    let r2 = &str;

    println!("r1 = {}", r1);
    println!("r2 = {}", r2);

    let r3 = &mut str;

    // Invalid
    // As str is already borrowed by r3 as mutable ref
    // let r4 = &mut str;

    println!("\nr3 = {}", r3);
    // println!("r4 = {}", r4);

    // r1, r2, r3 are out of scope now
    // since they're no longer used
    // Thus, r5 can be mutable
    // Because it's the only one in scope

    let r5 = &mut str;
    println!("\nr5 = {}", r5);

    // Mixing mutable and immutable also invalid
    // To prevent race condition
    let r6 = &str;
    // let r7 = &mut str;

    println!("\nr6 = {}\n", r6);
    // println!("\nr7 = {}", r7);

    // let ref_to_nothing = dangle();

    // println!("ref_to_nothing = {}", ref_to_nothing)
}

fn change_str(some_string: &mut String) {
    some_string.push_str(" - end");
}

// INVALID
// Here a reference to local value is returned
// Which will be dropped once the function goes out of scope
// fn dangle() -> &String {
//     let str = String::from("Probably Nothing");

//     &str
// }

/*************** SLICES ***************/

fn slices() {
    println!("\n------ SLICES ------\n");

    let mut str = String::from("Hello World");
    let word = first_word_normal(&str);
    println!("str = {}", str);
    println!("first word (normal) = {}", word);
    println!("....str cleared.....");

    str.clear();

    /*
       PROBLEM WITH THIS
       - str is cleared, but word still points to index 5
       - We need to manually keep str and word in sync
       - Thus, this method is error prone
       - We can solve this using slices
    */
    println!("str (cleared) = {}", str);
    println!("first word (normal) = {}\n", word);

    let mut str2 = String::from("Hey Rust");
    let word2 = first_word_slices(&str2);

    println!("str2 = {}", str2);
    println!("first word (slice) = {}", word2);

    /*
        ERROR HERE:
        - Slices are immutable by default
        - We have word2 as slice of str2
        - Thus, we cannot mutate it as long as word2 is in scope
        - This prevents runtime error and memory leaks
    */

    // str2.clear();

    println!("str (cleared) = {}", str2);
    println!("first word (normal) = {}", word2);

    /* Works since word2 is out of scope */
    str2.clear();
}

fn first_word_normal(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    str.len()
}

fn first_word_slices(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}

/*************** SLICES TYPES ***************/

fn slices_types() {
    println!("\n------ SLICES TYPES ------\n");

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    print_arr("[i to j] = ", &arr[3..6]);
    print_arr("[till i] = ", &arr[..6]);
    print_arr("[from i] = ", &arr[6..]);
    print_arr("[all] = ", &arr[..]);
}

fn print_arr(label: &str, arr: &[i32]) {
    let mut str = String::from(label);

    for i in arr {
        str.push_str(&i.to_string());
        str.push_str(", ");
    }

    println!("{}", str);
}
