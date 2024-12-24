
use std::fs;


fn main() -> std::io::Result<()> {
    // Load the file
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    // Separate into lines
    let mut lines = contents.lines();
    println!("With text:\n{contents}");

    let mut v_left: Vec<i32> = Vec::new();
    let mut v_right: Vec<i32> = Vec::new();
    // Iterate over lines to split into left and right vectors
    for l in lines {
        let mut x = l.split_whitespace();
        let mut y = x.next().unwrap().to_string();
        v_left.push(y.parse().unwrap());
        let mut z = x.next().unwrap().to_string();
        v_right.push(z.parse().unwrap());
    }

    // Sort both vectors
    v_left.sort();
    v_right.sort();

    // Absolute difference between the sorted vectors
    let mut v_diff: Vec<i32> = Vec::new();
    for (i, el) in v_left.iter().enumerate() {
        v_diff.push((*el - v_right[i]).abs());
    }

    // Sum the differences
    let mut total = 0;
    for i in &v_diff {
        total += i;
    }
    println!("{total}");

    Ok(())
}