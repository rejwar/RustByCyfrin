fn SafeAccess() {
    let arr = [10,20,30];

    if let Some(val) = arr.get(2){
        println!("Safe access {}", val);
    }
}

fn main() {
    SafeAccess();
}