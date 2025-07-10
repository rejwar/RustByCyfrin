fn ShowAllOverflow() {
    let x: u8 = 255;
    println!("Wrapping overflow {}", x.wrapping_add(1));
    println!("Checked : {:?}",x.checked_add(1));
    println!("Overflowing {:?}",x.overflowing_add(1));
    println!("Saturating {:?}",x.saturating_add(1));
}

fn main() {
    ShowAllOverflow();
}