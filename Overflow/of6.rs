fn DebugVsRelease() {
    let x: u8 = 255;

    let y = x.wrapping_add(1);
    println!("y = {} ",y);
}

fn main() {
    DebugVsRelease();
}