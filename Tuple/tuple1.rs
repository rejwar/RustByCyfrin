fn main() {
    let tuple = (42, "Rust", true);

    println!("First item {}", tuple.0);
    println!("Second item {}", tuple.1);
    println!("Third item {}", tuple.2);

    //pattern matching
    match tuple {
        (42, _,_) => println!("First time is 42"),
        (_,"Rust", _) => println!("Second item is rustc"),
        _ => println!("No match"),
    }
}