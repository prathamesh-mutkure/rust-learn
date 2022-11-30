fn main() {
    option_enum();
    pattern_matching();
    matching_option_enum();

    println!("---------------------------------------------");
}

fn option_enum() {
    println!("\n---------- NULL VALUES IN RUST ----------\n");

    /*
       OPTION ENUM
       - Rust has no null values
       - It has an alternative enum
       - Which is included in the default scope

       enum Option<T> {
           None,
           Some(T),
       }

       - The option variable need to be used considering both scenerios
       - Value being present as well as None
    */

    let x = 8;
    let y = Some(10);

    /*
       UNWRAP
       - The unwrap method returns the value stored in Some(T)
       - This method may panic if the value is null
       - This gives runtime error
       - Discouraged to use without handling None case
    */
    let sum = x + y.unwrap_or(0);

    println!("The sum of {} and {:?} is {}", x, y, sum);

    let y: Option<i32> = None;

    /*
       UNWRAPOR
       - The unwrapor method returns a default value if the variable is None
       - This prevents panicking
       - As both cases are handled through default value
    */
    let sum = x + y.unwrap_or(0);

    println!("The sum of {} and {:?} is {}", x, y, sum);
}

fn pattern_matching() {
    println!("\n---------- PATTERN MATCHING IN RUST ----------\n");

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let qtr = Coin::Quarter(UsState::Alaska);

    println!("A {:?} is worth {} cents", penny, value_in_cents(&penny));
    println!("A {:?} is worth {} cents", nickel, value_in_cents(&nickel));
    println!("A {:?} is worth {} cents", dime, value_in_cents(&dime));
    println!("A {:?} is worth {} cents", qtr, value_in_cents(&qtr));

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)]
    enum UsState {
        Alaska,
        California,
        Texas,
        Florida,
        // Others...
    }

    /*
        PATTERN MATCHING IN RUST
        - The match syntax can be used to return different values for an enum
        - Based on the enum type
        - Pattern matching is "exaustive" in rust
        - i.e. we need to define outcome of each and every type
        - Otherwise the code won't compile
        - Alternatively, use the wildcard "_" as default value for remaing
    */
    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => {
                print!("Lucky penny! ");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                print!("Quarter from {:?}: ", state);
                25
            }
        }
    }
}

fn matching_option_enum() {
    println!("\n---------- MATCHING OPTION ENUM ----------\n");

    let num = Some(10);

    println!("The value of num is {:?}", num);
    println!("...adding one");
    let num = add_one(num);
    println!("n value of num is {:?}\n", num);

    let num_none = None;

    println!("The value of num is {:?}", num_none);
    println!("...adding one");
    let num_none = add_one(num_none);
    println!("The value of num is {:?}\n", num_none);

    /* Adding one only if value exists, otherwise returning the same value */
    fn add_one(num: Option<i32>) -> Option<i32> {
        match num {
            Some(num) => Some(num + 1),
            _ => num,
        }
    }

    let some_value = Some(3);

    /*
        - Normal match syntax
        - It is exaustive in nature
        - You need to specify all possible values
        - This can be cumbersome if many values
    */
    match some_value {
        Some(3) => println!("The value is three"),
        _ => println!("The value is not three"),
    }

    /*
        IF LET SYNTAX
        - This can be used define behavior for just one type of enum
        - Thus, no need to specify all values
    */
    if let Some(3) = some_value {
        println!("It's three");
    }
}
