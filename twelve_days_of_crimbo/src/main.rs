fn main() {
    let my_list = vec!["this", "that", "chicken", "pie", "eight more things"];
    for (index, value) in my_list.iter().enumerate() {
        match index {
            1 => {
                println!("On the {}st day of christmas my true love gave to me", index+1);
                println!("{value}");
            }
            2 => {
                println!("On the {}nd day of christmas my true love gave to me", index+1);
                println!("{value}");
            }
            3 => {
                println!("On the {}rd day of christmas my true love gave to me", index+1);
                println!("{value}");
            }
            _ => {
                println!("On the {}th day of christmas my true love gave to me", index+1);
                println!("{value}");
            } 
        }
    }
}
