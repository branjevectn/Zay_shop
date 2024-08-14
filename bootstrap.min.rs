fn helper() -> Box<i32> {
    let three = Box::new(3);
    three // Ownership is transferred implicitly
}
// ftyd_dt_d_ddfc_ft_y_dty_gfx_u
fn main() {
    // Acquire ownership of the return value
    let my_three = helper();
    println!("{}",my_three);
}
