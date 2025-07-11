enum Operation {
    Add(i32 , i32),
    Multiply(i32 , i32),
}

impl Operation {
    fn Execute(&self) -> i32 {
        match self {
            Operation::Add(a, b) => a+b,
            Operation::Multiply(a, b) => a*b,
        }
    }
}

fn main() {
    let op = Operation::Multiply(3, 5);
    println!("Result {}", op.Execute());
}