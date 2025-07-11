fn sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
} 

fn main() {
    let data = [10,20,30,40];
    println!("Sum = {}", sum(&data));
}