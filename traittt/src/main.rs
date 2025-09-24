trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("Value: {}", self)
    }
}

fn print_all<T: Printable>(items: Vec<T>) {
    for item in items {
        item.print();
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    print_all(nums);
}
