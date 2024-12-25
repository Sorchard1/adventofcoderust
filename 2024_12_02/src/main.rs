
use std::fs;
use std::collections::HashMap;

fn is_increasing(vec: &mut Vec<i32>) -> bool {
    let mut increasing: bool = true;
    let mut decrease_found: bool;
    let mut more_than_three: bool;
    for (i, el) in vec.iter().take(vec.len() - 1).enumerate() {
        decrease_found = vec[i] >= vec[i + 1];
        more_than_three = (vec[i + 1] - vec[i]).abs() >3;
        if (decrease_found || more_than_three){
            increasing = false
        }
    }
    return increasing;
}

fn is_decreasing(vec: &mut Vec<i32>) -> bool {
    let mut decreasing: bool = true;
    let mut increase_found: bool;
    let mut more_than_three: bool;
    for (i, el) in vec.iter().take(vec.len() - 1).enumerate() {
        increase_found = vec[i] <= vec[i + 1];
        more_than_three = (vec[i + 1] - vec[i]).abs() > 3;
        if (increase_found || more_than_three){
            decreasing = false
        }
    }
    return decreasing;
}

fn main() -> std::io::Result<()> {
    // Load the file
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    // Separate into lines
    let mut lines = contents.lines();
    println!("With text:\n{contents}");

    let mut v_row: Vec<i32> = Vec::new();
    // Iterate over lines to split into left and right vectors
    let mut y: i32 = 0;
    let mut inc: bool;
    let mut dec: bool;
    let mut num_safe: i32 = 0;

    for l in lines {
        for (i, x) in l.split_whitespace().enumerate() {
            y = x.parse().unwrap();
            v_row.push(y);
        }
        inc = is_increasing(& mut v_row);
        dec = is_decreasing(& mut v_row);
        println!("increasing is {inc}");
        println!("decreasing is {dec}");
        if inc || dec {
            num_safe += 1;
        }
        v_row.clear();
    }

    println!("Part 1 is {num_safe}");

    // println!("Part 2 is {part_2}");
    Ok(())
}