fn HeapBox() {
    let b = Box::new(100);
    println!("b = {}", b);
}

fn main() {
    HeapBox();
}