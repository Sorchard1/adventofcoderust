
use std::fs;
use std::collections::HashMap;

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

    let mut v_left_original: Vec<i32> = Vec::new();
    let mut v_right_original: Vec<i32> = Vec::new();
    for (i, el) in v_left.iter().enumerate() {
        v_left_original.push(*el);
    }
    for (i, el) in v_right.iter().enumerate() {
        v_right_original.push(*el);
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
    println!("Part 1 is {total}");

    let mut m: HashMap<i32, usize> = HashMap::new();
    for x in v_right_original {
        *m.entry(x).or_default() += 1;
    }
    let mut part_2: i32 = 0;
    for x in v_left_original {
        let t:i32;
        let ss:i32;

        if (m.contains_key(&x)) == true {
            t = m[&x] as i32;
        }
        else {
            t = 0;
        }
        // println!("{t}");
        ss = t*x;

        part_2 +=ss;
    }
    println!("Part 2 is {part_2}");
    Ok(())
}