use std::io;
fn main() {
    println!("By how much do you want to increment the number?");
    let mut total_value: i32 = 0;
    loop {
        println!("Current: {total_value}. Increment by:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error!");
        let input_value: i32 = input.trim().parse().expect("Parse i32 error!");

        if input_value == 0 {
            break;
        } else if input_value < 0 {
            println!("Value must be positive number!");
            continue;
        } else {
            total_value += input_value;
            if total_value > i16::MAX.into() {
                println!("Enough incrementations.");
                break;
            }
        }
    }
}
