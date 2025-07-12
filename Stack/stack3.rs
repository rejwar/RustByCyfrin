use std::mem::size_of_val;

fn MemorySizes() {
    let stack_val =100;
    let heap_val = Box::new(100);

    println!("Stack value {} bytes " , size_of_val(&stack_val));
    println!("Box pointer size (on stack ) : {} bytes " ,size_of_val(&heap_val));   
}

fn main() {
    MemorySizes();
}