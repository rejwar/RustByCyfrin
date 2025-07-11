enum Expr {
    Val( i32 ),
    Add(Box<Expr> , Box<Expr>),
}

fn Eval (expr: &Expr) -> i32  {
    match expr {
        Expr::Val(x) => *x,
        Expr::Add(a, b) => Eval(a) + Eval(b),
    }
}

fn main () {
    let expr = Expr::Add(Box::new(Expr::Val(2)),Box::new(Expr::Val(3)));
    println!("Eval result = {}", Eval(&expr));
}
