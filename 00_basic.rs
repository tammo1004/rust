fn main() {
    const PI: f64 = 3.14;
    println!("{PI}");

    let x: u32 = 3;
    println!("{x}");

    {
        let x: u32 = 5;
        println!("{x}");
    }
    println!("{x}");

    let mut y: u32 = 7;
    println!("{y}");

    y = 9;
    println!("{y}");
}