
fn say_hello_by_value(name: String) {
    println!("Hello {}!", name);
}

fn say_hello_by_reference(name: &String) {
    println!("Hello {}!", name);
}

fn main() {
    let name = String::from("Jim");
    say_hello_by_reference(&name);
    say_hello_by_value(name);

    // Can't use a value that's already been moved
    // say_hello_by_value(name);
}
