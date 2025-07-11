fn main() {
    let numbers = vec![3,4,5,6,7,8,9];

    println!("First Item {}", numbers[0]);


    match numbers.get(2) {
        Some(value) => println!("Third item {}", value),
        None => println!("Index out of bounds"),
    }
}