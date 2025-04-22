fn main() {
    // 1.相互转换摄氏温度和华氏温度
    let celsius = 20.0;
    let fahrenheit = convert_to_fahrenheit(celsius);
    println!("{}°C is {}°F", celsius, fahrenheit);

    let fahrenheit = 68.0;
    let celsius = convert_to_celsius(fahrenheit);
    println!("{}°F is {}°C", fahrenheit, celsius);

    // 2. 计算斐波那契数列
    let n = 10;
    let fibonacci = fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, fibonacci);

    //3.打印 The Twelve Days of Christmas 歌词
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for i in 0..days.len() {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );
        for j in 0..i + 1 {
            println!("{}", gifts[j]);
        }
        println!();
    }
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
