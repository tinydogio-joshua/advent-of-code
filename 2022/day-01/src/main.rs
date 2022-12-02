// Day 1: I'm sure this can be done better, but it's my first attempt at Rust.

use std::fs;

#[derive(Debug)]
struct Elf {
    food: Vec<Food>,
    total_calories: i32,
}

#[derive(Debug)]
struct Food {
    calories: i32,
}

fn create_elf(data: &str) -> Elf {
    let food: Vec<Food> = data
        .split("\n")
        .map(|line: &str| Food { calories: line.parse().unwrap() })
        .collect();
    let total_calories = food.iter().fold(0, |acc, item| acc + item.calories);
    Elf { food: food, total_calories: total_calories }
}

fn main() {
    match fs::read_to_string("./data") {
        Ok(data) => process_data(data),
        Err(_) => println!("Unable to Read File \"./data\"."),
    }
}

fn process_data(data: String) {
    let mut elves: Vec<Elf> = data
        .trim()
        .split("\n\n")
        .map(|elf| create_elf(elf))
        .collect();
 
    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    println!("{:?}", elves);
    println!("===================================");
    println!("Part A: {:?}", elves[0].total_calories);
    println!("Part B: {:?}", elves.iter().take(3).map(|elf| elf.total_calories).sum::<i32>());
}

