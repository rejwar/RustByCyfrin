fn MapArray() {
    let arr = [1,2,3,45,6];

    let squared: Vec<i32> = arr.iter().map(|x| x *x ).collect();
    println!("Squrad : {:?}", squared);
}

fn main() {
    MapArray();
}