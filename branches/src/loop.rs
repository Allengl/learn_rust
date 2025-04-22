fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    'counting_up: loop {
        println!("counting up: {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
    }
    println!("Encountered an error, exiting.");

    // 用 if 实现  while 循环
    let mut a = 0;
    loop {
        a += 1;
        if a == 10 {
            break;
        }
    }

    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("element: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}
