#![allow(unused)]

fn add_wit_return(x: i32 , y: i32) -> i32 {
    return x +y;
}

fn add(x: u32 , y: u32 ) -> u32 {
    x + y
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}" );
}