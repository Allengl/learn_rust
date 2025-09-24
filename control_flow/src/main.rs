use std::io;

fn main() {
    println!("请输入斐波那契数列的长度 n: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("请输入有效的正整数");
            return;
        }
    };

    if n == 0 {
        println!("斐波那契数列前 0 个数字：");
        return;
    }

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut fib_sequence = vec![]; // 修复1：初始化为空向量

    for _ in 0..n {
        // 修复2：循环从0到n-1
        fib_sequence.push(a.to_string());
        let next = a + b;
        a = b;
        b = next;
    }

    // 修复3：用空格分隔数字
    println!("斐波那契数列前 {} 个数字：{}", n, fib_sequence.join(" "));

    println!("5 是 {}", is_even_or_odd(5));
}

fn is_even_or_odd(n: i32) -> &'static str {
    match n % 2 {
        0 => "偶数",
        _ => "奇数",
    }
}

