use std::io;
fn main() {
    let var1 = "Rust";
    let var2 = "No";

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let input = input.trim();

    if input == var1 {
        println!("So you appreciate Rust? That's great! Thank you!");
    } else if input == var2 {
        println!("So you like nothing? Alright then... :)");
    } else {
        println!("It seems that you like {input}");
    }
}
