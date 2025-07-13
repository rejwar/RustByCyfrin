fn main() {
    let Coordinates =(10,20);
    let (X,Y) = Coordinates;

    println!("X is {}", X);
    println!("Y is {}",Y);

    PrintPoint((5,10));
}

    fn PrintPoint((X ,Y): (i32 , i32)) {
        println!("Point is at ({} , {})" , X ,Y);
    }