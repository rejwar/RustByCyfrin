fn main() {
    let x: u8 = 255;
    let y = x.saturating_add(1);

    println!("Result {}", y);
}