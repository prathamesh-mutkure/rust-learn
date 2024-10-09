fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    println!("-------- iter.next() --------");

    let mut colors_iter = colors.iter();

    print!("{:?} ", colors_iter.next());
    print!("{:?} ", colors_iter.next());
    print!("{:?} ", colors_iter.next());
    print!("{:?} ", colors_iter.next());
    println!();

    print_elements(&colors);
    print_elements_uc(&colors);

    shorten_color(&mut colors);
    print_elements(&colors);

    print_elements(&to_uppercase(&colors));

    let mut colors2: Vec<String> = vec![];

    move_vec(colors, &mut colors2);
    print_elements(&colors2);

    // print!("{:#?}", colors);

    println!("{:?}", &explode(&colors2));

    println!("{:?}", find_or(&colors2, "r", "not found"));
}

/* Print each element one by one (for loop) */
fn print_elements(elements: &Vec<String>) {
    println!("-------- print_elements --------");
    for element in elements {
        print!("{:?} ", element);
    }
    println!();
}

/* Print each element one by one (consumers and adapters) */
fn print_elements_uc(elements: &[String]) {
    println!("-------- print_elements_uc --------");

    // .map => adapter
    // .for_each => consumer

    // Iterators are lazy
    // .map() does not execute until .for_each() is called
    // Till then no effect
    // i.e. adapters don't call .next() on the iterator

    elements
        .iter()
        .map(|el| el.to_uppercase())
        .for_each(|el| print!("{:?} ", el));

    println!();
}

/* Print each element one by one (for loop) */
fn shorten_color(elements: &mut [String]) {
    println!("-------- shorten_color --------");

    elements
        .iter_mut()
        .map(|el| el.truncate(3))
        .for_each(|el| print!("{:?} ", el));

    println!();
}

/* Return a new vector with uppercase strings */
fn to_uppercase(elements: &[String]) -> Vec<String> {
    println!("-------- to_uppercase --------");

    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<_>>()
}

/* Move vales from vec_a to vec_b */
fn move_vec(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    println!("-------- move_vec --------");
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

/* Explode, convert Vec<String> to Vec<Vec<String>> */
fn explode(elements: &[String]) -> Vec<Vec<String>> {
    println!("-------- explode --------");

    elements
        .iter()
        .map(|el| el.chars().map(|ch| ch.to_string()).collect::<Vec<_>>())
        .collect::<Vec<Vec<String>>>()
}

/* Find matching element or return a default */
fn find_or(elements: &[String], search: &str, default: &str) -> String {
    println!("-------- find_or --------");

    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(default), |el| el.to_string())
}
