use std::fs;


fn is_increasing(vec: &mut Vec<i32>) -> bool {
    let mut increasing: bool = true;
    let mut decrease_found: bool;
    let mut more_than_three: bool;
    for (i, _el) in vec.iter().take(vec.len() - 1).enumerate() {
        decrease_found = vec[i] >= vec[i + 1];
        more_than_three = (vec[i + 1] - vec[i]).abs() > 3;

        if decrease_found || more_than_three {
            increasing = false;
            break;
        }
    }
    return increasing;
}

fn is_decreasing(vec: &mut Vec<i32>) -> bool {
    let mut decreasing: bool = true;
    let mut increase_found: bool;
    let mut more_than_three: bool;
    for (i, _el) in vec.iter().take(vec.len() - 1).enumerate() {
        increase_found = vec[i] <= vec[i + 1];
        more_than_three = (vec[i + 1] - vec[i]).abs() > 3;

        if increase_found || more_than_three{
            decreasing = false;
            break;
        }
    }
    return decreasing;
}

fn clone_without(vec: &Vec<i32>, index:usize) -> Vec<i32> {
    let mut y: Vec<i32> = Vec::new();
    for (i, el) in vec.iter().enumerate() {
        if i != index {
            y.push(*el);
        }
    }
    return y;
}

fn is_increasing_one_error(vec: &mut Vec<i32>) -> bool {
    let mut increasing: bool = true;
    let mut decrease_found: bool;
    let mut more_than_three: bool;
    for (i, _el) in vec.iter().take(vec.len() - 1).enumerate() {
        decrease_found = vec[i] >= vec[i + 1];
        more_than_three = (vec[i + 1] - vec[i]).abs() > 3;

        if decrease_found || more_than_three {
            let mut v_1: Vec<i32> = clone_without(vec, i);
            increasing = is_increasing(& mut v_1);
            if !increasing {
                let mut v_2: Vec<i32> = clone_without(vec, i+1);
                increasing = is_increasing(& mut v_2);
            }
            break;
        }
    }
    return increasing;
}
fn is_decreasing_one_error(vec: &mut Vec<i32>) -> bool {
    let mut decreasing: bool = true;
    let mut increase_found: bool;
    let mut more_than_three: bool;

    for (i, _el) in vec.iter().take(vec.len() - 1).enumerate() {
        increase_found = vec[i] <= vec[i + 1];
        more_than_three = (vec[i + 1] - vec[i]).abs() > 3;
        if increase_found || more_than_three {
            decreasing = is_decreasing(& mut clone_without(vec, i));
            if !decreasing {
                decreasing = is_decreasing(& mut clone_without(vec, i+1));
            }
            break;
        }
    }
    return decreasing;
}

fn main() -> std::io::Result<()> {
    // Load the file
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    // Separate into lines
    let lines = contents.lines();
    // println!("With text:\n{contents}");

    let mut v_row: Vec<i32> = Vec::new();
    // Iterate over lines to split into left and right vectors
    let mut y: i32;
    let mut inc: bool;
    let mut dec: bool;
    let mut inc_one_allowed: bool;
    let mut dec_one_allowed: bool;
    let mut num_safe: i32 = 0;
    let mut num_safe_one_allowed: i32 = 0;

    for l in lines {
        for (_i, x) in l.split_whitespace().enumerate() {
            y = x.parse().unwrap();
            v_row.push(y);
        }
        inc = is_increasing(& mut v_row);
        dec = is_decreasing(& mut v_row);
        inc_one_allowed = is_increasing_one_error(& mut v_row);
        dec_one_allowed = is_decreasing_one_error(& mut v_row);

        if inc || dec {
            num_safe += 1;
        }
        if inc_one_allowed || dec_one_allowed {
            num_safe_one_allowed += 1;
        }
        v_row.clear();
    }

    println!("Part 1 is {num_safe}");

    println!("Part 2 is {num_safe_one_allowed}");
    Ok(())
}