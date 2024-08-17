fn main() {
    println!("Hello, world!");
    another_function(3);
    print_labeled_measurement(5, 'h');
    let x = plus_one(5);
    println!("x is {x}");
}

fn another_function(x: i32) {
    println!("Another function");
    println!("This function prints {x}");
}

fn print_labeled_measurement(size: i32, unit: char) {
    println!("The measurement is {size}{unit}")
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
