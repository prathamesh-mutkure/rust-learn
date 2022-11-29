fn main() {
    println!("\n---------- STRUCT IN RUST ----------\n");

    let rect_small = Rectangle {
        height: 30,
        width: 50,
    };

    let react_large = Rectangle {
        height: 40,
        width: 80,
    };

    /* The .. syntax can be used to copy remaining properties */
    let react_like_large = Rectangle {
        height: 50,
        ..react_large
    };

    /* Unlink C++, members of references can be directly accessed by the dot syntax */
    /* This is done by rust's automatic referencing and dereferencing */
    println!("The area of {:?} is: {}", &rect_small, &rect_small.area());
    println!("The area of {:?} is: {}", &react_large, react_large.area());
    println!(
        "The area of {:?} is: {}",
        &react_like_large,
        react_like_large.area()
    );

    println!(
        "\nCan {:?} hold {:?} ? The answer is -> {}\n",
        react_large,
        rect_small,
        react_large.can_hold(&rect_small)
    );

    /* Associated method, called using :: notation */
    let rect_sq = Rectangle::square(10);

    println!("This {:?} is basically square", &rect_sq);

    println!();
    println!("\n---------- STRUCT IN RUST ----------\n");
}

/* This trait is used so that compiler creates default implementation for printing the struct */
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
    // color: String,
}

/*
    IMPLEMENTING STRUCS
    - Structs can be associated with methods by implementing them
    - The first argument &self is a reference to current object
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.height > other.height) && (self.width > other.width)
    }
}

/*
    ASSOCIATED METHODS
    - Associated methods are related to struct not instance
    - Like static methods in other languages
    - No &self reference

    - Usually declared in seperate implementaion
    - Called using :: notation
*/
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}
