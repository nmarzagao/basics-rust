use rand::Rng;
use std::io;

fn main() {
    let dice_str = get_input();
    let number = parse_input(dice_str);
    
    let roll = roll_dice(number);
    println!("You rolled a {}", roll);
}

fn get_input() -> String {
    println!("What die would you like to roll? (d2,d6,d4...)");
    
    let mut dice_str = String::new();

    io::stdin()
        .read_line(&mut dice_str)
        .expect("Failed to read line");

    return dice_str
}

fn parse_input(input: String) -> u32 {
    let mut number_str = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            number_str = format!("{}{}", number_str, c)
        }
    }

    let number: u32 = match number_str.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input is not a number")
    };

    return number
}

fn roll_dice(max: u32) -> u32 {
    let mut rng = rand::thread_rng();

    return rng.gen_range(1..max)
}