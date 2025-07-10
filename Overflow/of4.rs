fn ChkAdd(a: u8, b: u8) {
    match a.checked_add(b) {
        Some(sum) => println!("Sum {}", sum),
        None => println!("Overflow detected"),
    }
}

fn main() {
    ChkAdd(100, 27);
    ChkAdd(200, 100);
}