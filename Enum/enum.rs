enum Shape {
    Circle(f64),
    Rectangle {w: f64 , h: f64},
}

fn DescribeShape(shape: Shape) {
    match shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle { w, h } => println!("Rectangle {} x {}", w,h),
    }
}

fn main() {
    DescribeShape(Shape::Circle(2.5));
    DescribeShape(Shape::Rectangle { w: 4.0, h: 5.0 });
}