fn processData(data: &(i32 , f64 , &str)) {
    let (id , score , name) = data;
    println!(" Id: {} , Score {} , Name {}", id , score, name);
}

fn main() {
    let data = &(1 , 95.5, "ALICE");
    processData(data);
}