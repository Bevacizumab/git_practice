fn substract(a: i32, b:i32) -> i32 {
    a - b
}

fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");

    let addition = add(3,4);
    println!("Add: {addition}");

    let substraction = substract(3, 4);
    println!("Substraction: {substraction}");
}
