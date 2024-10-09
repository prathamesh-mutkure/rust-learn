mod basket;
mod container;
mod stack;

use std::string;

use basket::Basket;
use container::Container;

use num_traits::ToPrimitive;
use stack::Stack;

fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f32 = a.to_f64().unwrap();
    let b_f32 = b.to_f64().unwrap();

    (a_f32.powi(2) + b_f32.powi(2)).sqrt()
}

fn add_string<T: Container<String>>(container: &mut T, string: String) {
    container.put(string);
}

fn main() {
    let a: f64 = 3.0;
    let b: i32 = 4;

    // println!("{}", solve(a, b));

    let mut b1 = Basket::new(String::from("Hi"));

    let mut s1 = Stack::new(vec![String::from("Hi")]);

    add_string(&mut b1, String::from("Hola"));
    add_string(&mut s1, String::from("Hola"));

    println!("{:?}", b1);
    println!("{:?}", s1);
}
