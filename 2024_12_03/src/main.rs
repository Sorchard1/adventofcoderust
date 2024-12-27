use std::fs;
use regex::Regex;
fn search_and_count(haystack: &str) ->i32 {
    let pattern: &str = r"mul\([0-9]+,[0-9]+\)";
    let re = Regex::new(pattern).unwrap();
    let occurrences: Vec<&str> = re.find_iter(haystack).map(|m| m.as_str()).collect();

    let subpattern: &str = r"[0-9]+";
    let sub_re = Regex::new(subpattern).unwrap();
    let mut total: i32 = 0;

    for occurrence in occurrences {
        let values: Vec<&str> = sub_re.find_iter(occurrence).map(|m| m.as_str()).collect();
        total += (values[0].parse::<i32>().unwrap()) * (values[1].parse::<i32>().unwrap());
    }
    return total;
}


fn main() -> std::io::Result<()> {
    // Load the file
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");
    let total = search_and_count(&contents);
    println!("Part 1 is {total}");

    let mut after2: String = "".to_owned();
    let mut borrowed_string: &str;
    let mut state: bool = true;
    let mut index_s: usize = 0;
    let mut found_dont: bool;
    let mut found_do: bool;

    for (i, _c) in contents.chars().enumerate() {
        if i > 6 {
            if state {
                borrowed_string = &contents[i-7..i];
                found_dont = borrowed_string == "don't()";
                if found_dont {
                    after2.push_str(&contents[index_s..i-7]);
                    state = false;
                }
            }
            else {
                borrowed_string = &contents[i-4..i];
                found_do = borrowed_string == "do()";
                if found_do {
                    state = true;
                    index_s = i;
                }
            }
        }
    }

    // let after = Regex::new(r"don't\(\).*?do\(\)").unwrap().replace_all(&contents, "");
    // let exp = Regex::new(r"don't\(\).*").unwrap().replace_all(&after, "");
    let total2 = search_and_count(&after2);
    println!("Part 2 is {total2}");
    Ok(())
}