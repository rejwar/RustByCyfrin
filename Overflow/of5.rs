fn chkAdd(a: u8 , b: u8) {
    match a.checked_sub(b) {
        Some(diff) => println!("Difference {}", diff),
        None => println!("Underflow detected"),
    }
}

fn main() {
    chkAdd(50, 30);
    chkAdd(20, 30);
}