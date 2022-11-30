fn main() {
    println!("\n---------- ENUMS IN RUST ----------\n");

    let ip = IPAddr {
        kind: IPAddrKind::IPv4,
        address: String::from("127:0:0:0"),
    };

    println!("IP Address is: {:?}\n", ip);

    let ip2 = IPAddrInt::IPv6(String::from("127:0:0:0"));
    println!("IP Address is: {:?}\n", ip2);

    println!();
}

/* Normal Enum in Rust */
#[derive(Debug)]
enum IPAddrKind {
    IPv4,
    IPv6,
}

/* Adding Enum to struct */
#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

/* Storing data directly in Enum */
#[derive(Debug)]
enum IPAddrInt {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

/*
   ENUMS WITH DATA
   - Here Quit, Move, Write and ChangeColor could have been declared as seperate structs
   - With their own methods
   - Enums help you to group them into single enum
*/

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Enums with Data in Rust");
    }
}
