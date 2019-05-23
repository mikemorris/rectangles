fn main() {
    let rect = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(rect));
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
