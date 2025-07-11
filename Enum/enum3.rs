enum Grade {
    Score(i32),
}

fn CheckGrade(g: Grade) {
    match g {
        Grade::Score(x) if x >= 90 => println!("Excellent"),
        Grade::Score(x ) if x >= 50 => println!("Pass"),
        Grade::Score(_) => println!("Fail"),
    }
}


fn main() {
    CheckGrade(Grade::Score(95));
}