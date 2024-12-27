use std::fs;
use regex::Regex;

fn main() -> std::io::Result<()> {
    // Load the file
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");
    let pattern: &str = r"mul\([0-9]+,[0-9]+\)";
    let re = Regex::new(pattern).unwrap();
    let haystack: &str = &contents;
    let occurrences: Vec<&str> = re.find_iter(haystack).map(|m| m.as_str()).collect();

    let subpattern: &str = r"[0-9]+";
    let sub_re = Regex::new(subpattern).unwrap();
    let mut total: i32 = 0;

    for occurrence in occurrences {
        let values: Vec<&str> = sub_re.find_iter(occurrence).map(|m| m.as_str()).collect();
        total += (values[0].parse::<i32>().unwrap()) * (values[1].parse::<i32>().unwrap());
    }
    println!("Part 1 is {total}");

    Ok(())
}