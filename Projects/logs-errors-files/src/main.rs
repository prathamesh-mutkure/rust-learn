use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    /* unwrap, expect, unwrap_or */
    /* Not recommended in prod */
    // let file_data_1 = fs::read_to_string("logs.txt").expect("Failed to read file");
    // let error_logs_1: Vec<String> = extract_errors(&file_data_1);
    // fs::write("error_logs.txt", error_logs_1.join("\n")).expect("Failed to write to filw");

    /* Try Operator */
    /* Extracts value if all good */
    /* Else exits main with error */
    // let file_data_1 = fs::read_to_string("logs.txt")?;
    // let error_logs_1: Vec<String> = extract_errors(&file_data_1);
    // fs::write("error_logs.txt", error_logs_1.join("\n"))?;

    /* Nested Matches */
    let file_data = fs::read_to_string("logs.txt");
    let mut error_logs: Vec<String> = vec![];

    match file_data {
        Ok(data) => {
            error_logs = extract_errors(&data);

            match fs::write("error_logs.txt", error_logs.join("\n")) {
                Ok(..) => println!("Error logs written to file"),
                Err(err) => println!("Error writing file: {}", err),
            }
        }
        Err(err) => println!("Error reading file: {}", err),
    }

    println!("{:#?}", error_logs);

    /* Ignore below code, for learning and testing */
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    match validate_email(String::from("test_test.com")) {
        Ok(..) => println!("Email is valid"),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}

fn extract_errors(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with("ERROR"))
        .map(|line: &str| line.to_string())
        .collect::<Vec<String>>()
}

fn test_string(a: String, b: &String, c: &str) {}

fn divide(a: f32, b: f32) -> Result<f32, Error> {
    if b == 0.0 {
        Err(Error::other("Zero division error"))
    } else {
        Ok(a / b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Email must contain an @"))
    }
}
