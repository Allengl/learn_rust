fn main() {
    let num = 10;

    // 使用 if-else 表达式进行条件判断
    if num > 0 {
        println!("Positive");
    } else if num < 0 {
        println!("Negative");
    } else {
        println!("Zero");
    }

    // 使用循环打印数字
    for i in 1..=5 {
        println!("{}", i);
    }

    // 使用 match 表达式匹配值
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }
}
