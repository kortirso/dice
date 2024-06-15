use std::io;
use rand::prelude::*;

fn parse_string(value: &str) -> i32 {
    value.parse().expect("Failed to parse string to integer")
}

fn main() {
    println!("Specify dices throw like 2d12, d20, 4d6");

    let mut multiplier: i32;
    let mut dice_value: i32;
    let mut result: i32;
    let mut throws: Vec<i32>;
    let mut throw_iterator: i32;

    loop {
        let mut dice = String::new();
        io::stdin().read_line(&mut dice).expect("Can't read line");

        let splitted: Vec<&str> = dice.trim().split("d").collect();

        multiplier = if splitted[0] == "" { 1 } else { parse_string(splitted[0]) };
        dice_value = parse_string(splitted[1]);
        throws = vec![];
        throw_iterator = 0;

        while throw_iterator < multiplier {
            throws.push(rand::thread_rng().gen_range(1..dice_value));
            throw_iterator = throw_iterator + 1;
        }

        result = throws.iter().sum();
        println!("Result: {}, throws: {:?}", result, throws);
    }
}
