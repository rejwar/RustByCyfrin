fn main() {
    let x: u8 =255;

    match x.checked_add(1) {
        Some(result ) => println!("Result : {}",result),
        None => println!("Overflow occured "),
    }
}