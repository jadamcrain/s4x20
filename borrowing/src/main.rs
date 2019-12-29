
struct Value {
    inner : i32
}

fn select_largest<'a>(a: &'a Value, b: &'a Value) -> &'a Value {
    if a.inner > b.inner { a } else { b }
}

/* uncomment me
fn print_by_value(value : Value) {
    println!("value is {}", value.inner);
}
*/

fn main() {
    let x = Value { inner: 42 };
    let y = Value { inner: 77 };

    let largest = select_largest(&x, &y);
    /* uncomment me
       The compiler won't let us 'move' either x or y and then later use the borrowed value
       print_by_value(x);
    */
    println!("Largest is {}", largest.inner);
}
