fn helper() -> Box<i32> {
    let three = Box::new(3);
    three // Ownership is transferred implicitly
}
// _____s___z____
fn main() {
    // Acquire ownership of the return value
    let my_three = helper();
    println!("{}",my_three);
}
