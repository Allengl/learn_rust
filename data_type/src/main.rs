struct Person {
    name: String,
    age: i32
}

enum TrafficLight {
    Red,
    Green,
    Yellow
}

fn main() {
  let person1 = Person{
        name: String::from("Alice"),
        age:30,
    };

    println!("Name:{}",person1.name);
    println!("Age:{}",person1.age);


   let current_light = TrafficLight::Red;

    match current_light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Green => println!("Go!"),
        TrafficLight::Yellow => println!("Caution"),
    }

    area()
}

fn area() {
    const PI: f64 = 3.14159;
    let radius: f64 = 2.0;
    let area = PI * radius * radius;
    println!("The area of the circle is: {}", area);
}

fn swap() {
    let mut a: i32 = 5;
    let mut b: i32 = 10;
    println!("Before swap: a = {}, b={}", a, b);
    let temp = a;
    a = b;
    b = temp;
    println!("After swap: a = {}, b = {}", a, b);
}

fn save() {
    let person: (&str, i32, &str) = ("John", 30, "Developer");
    println!("Name: {}, Age: {}, Occupation: {}", person.0, person.1, person.2);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First number: {}", numbers[0]);
}


