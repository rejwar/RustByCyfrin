#![allow(unused)]

fn main() {
    let mut x = u32::MAX;
    X += 1;

    println!("u32 max: {} , x : {}" , u32::MAX, X);

    let x = u32::Checked_add(u32::MAX , 1);
    println!("Checked_add: {?}",X);

    let x = u32::wrappig_add(u32::MAX,1);
    println!("wrapping add : {:?}",X);
}