use rand::Rng;

fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        let random_number = receive_random();
        match random_number {
            1..=3 => println!("Low.."),
            4..=6 => println!("Middle!"),
            7..=9 => println!("High!!"),
            10 => {
                println!("Jackpot!!!");
                break;
            }
            _ => println!("Uncovered")
        }
    }
    println!("{}", measure_luck(counter));
}

fn receive_random() -> i32 {
    let rng = rand::rng;
    rng().random_range(1..=10)
}
fn measure_luck(n: i32) -> String {
    if n > 3 {
        "You were UNLUCKY!".to_string()
    } else {
        "You were LUCKY!".to_string()
    }
}
