// Day 1: I'm sure this can be done better, but it's my first attempt at Rust.

use std::fs;

#[derive(Debug)]
struct Elf {
    food: Vec<Food>,
    total_calories: u32,
}

#[derive(Debug)]
struct Food {
    calories: u32,
}

fn create_elf(data: &str) -> Elf {
    let food: Vec<Food> = data
        .split("\n")
        .map(|line: &str| Food {
            calories: line.parse().unwrap(),
        })
        .collect();
    let total_calories = food.iter().map(|item| item.calories).sum::<u32>();
    Elf {
        food: food,
        total_calories: total_calories,
    }
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

    let max_calories = elves.iter().map(|elf| elf.total_calories).max().unwrap();
    let sum_top_three = elves
        .iter()
        .take(3)
        .map(|elf| elf.total_calories)
        .sum::<u32>();

    println!("{:?}", elves);
    println!("===================================");
    println!("Part A: {:?}", max_calories);
    println!("Part B: {:?}", sum_top_three);
}
