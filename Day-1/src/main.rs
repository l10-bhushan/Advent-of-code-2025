#![allow(warnings)]

use std::env;
use std::fs;

const MAX_VALUE: i16 = 99;

fn move_left(current_dial_pos: &mut i16, numbers_to_move: i16, dial_set_to_0: &mut Vec<i32>) {
    if *current_dial_pos == 0 {
        dial_set_to_0.push(1);
    }
    let mut newposition;
    if *current_dial_pos > numbers_to_move {
        newposition = *current_dial_pos - numbers_to_move;
    } else {
        newposition = numbers_to_move - *current_dial_pos;
        newposition = 100 - newposition;
    }
    if newposition == 100 {
        newposition = 0;
    }
    *current_dial_pos = newposition;
}

fn move_right(current_dial_pos: &mut i16, numbers_to_move: i16, dial_set_to_0: &mut Vec<i32>) {
    if *current_dial_pos == 0 {
        dial_set_to_0.push(1);
    }
    let mut newposition;
    newposition = *current_dial_pos + numbers_to_move;
    if newposition > 99 {
        let buffer = newposition - 100;
        newposition = buffer;
    } else if newposition == 100 {
        newposition = 0;
    }
    *current_dial_pos = newposition;
}

fn break_lock(input_vec: Vec<(&str, &str)>) -> usize {
    println!("Input vector is : {:?}", input_vec);
    let mut dial_set_to_0 = Vec::new();
    let mut dial_starts_at = 50;
    for i in 0..input_vec.len() {
        let direction = input_vec[i].0;
        let move_value: i16 = input_vec[i]
            .1
            .trim()
            .parse()
            .expect("Error: failed to parse value");
        match direction {
            "L" => move_left(&mut dial_starts_at, move_value, &mut dial_set_to_0),
            "R" => move_right(&mut dial_starts_at, move_value, &mut dial_set_to_0),
            _ => println!("Error"),
        }
    }
    dial_set_to_0.len()
}

fn main() {
    println!("Advent of code - Day 1");
    let args: Vec<String> = env::args().collect();
    println!("Filename is : {}", &args[1]);
    let file =
        fs::read_to_string("/Users/bhush98/Developer/Advent_of_code_2025/Day-1/src/input.txt")
            .expect("Error: Not able to read file");
    let lines = file.lines();
    let string_vec: Vec<(&str, &str)> = lines
        .map(|item| {
            let split_str = item.split_at(1);
            split_str
        })
        .collect();

    println!("Password is : {}", break_lock(string_vec));
}
