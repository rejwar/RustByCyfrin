struct Person {
    name: String,
    age: u8,
}

fn main() {
    let P = Person {
        name : String::from("Alice"),
        age: 30,
    };

    println!("Name: {},  age {}", P.name, P.age);
}