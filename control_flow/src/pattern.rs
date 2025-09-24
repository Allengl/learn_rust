fn main() {
    let number = 7;

    match number {
        1 => print!("One"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        10..=20 => println!("Between 10 and 20"),
        _ => println!("Other"),
    }
}
