fn main() {
    let mut num = 3;

    while num > 0 {
        println!("Countdown: {}", num);
        num -= 1;
    }

    println!("Liftoff");

    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("Element:{}", element);
    }
}
