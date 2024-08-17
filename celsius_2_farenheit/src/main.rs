use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value: f64 = args[1].trim().parse().expect("This should be a number");
    let unit: &str = args[2].trim();

    if unit == "farenheit" {
        let result = farenheit_to_cels(value);
        println!("{result} celsius");
    } else if unit == "celsius" {
        let result = cels_to_farenheit(value);
        println!("{result} farenheit");
    } else {
        println!("I need celsius or farenheit");
    }
}

fn farenheit_to_cels(farenheit: f64) -> f64 {
    return (farenheit - 32.0) / 1.8;
}

fn cels_to_farenheit(cels: f64) -> f64 {
    return cels * 1.8 + 32.0;
}
