use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: u64 = args[1].trim().parse().expect("need a number");
    let result = fibonacci(arg);
    println!("Result: {}", result)
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n - 2) + fibonacci(n - 1);
    };
}
