use rand::Rng;

fn one_function(x: i32) {
    println!("One function: {}", x);
}

fn two_function(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // Числові типи даних:
    let _x: i32 = 42;
    let _y: u64 = 100;
    let _pi: f64 = 3.14159;
    let _e: f32 = 2.71828;

    // Логічні типи даних:
    let _is_valid: bool = true;
    let _is_ready: bool = false;

    // Рядкові типи даних:
    let _name: &str = "John";
    let _message: String = String::from("Hello, Rust!");

    // Символьний тип даних:
    let _letter: char = 'A';
    let _emoji: char = '😀';

    //  Кортежі (Tuples)
    let person: (String, i32, bool) = (String::from("Сергій"), 30, true);
    let (_name, _age, _active) = person; // деструктуризація
    println!("Вік: {}", person.1); // доступ по індексу

    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // фіксований розмір
    let _zeros = [0; 10]; // 10 нулів

    println!("Перший елемент: {}", numbers[0]);

    let data = ("test", 100, 3.14);

    println!("{:?}", data);

    let test_random = rand::thread_rng().gen_range(1..100);

    println!("Random number: {}", test_random);

    one_function(120);

    let result = two_function(10, 20);

    println!("Result: {}", result);
}
