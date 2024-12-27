use std::fs;
fn down_found(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    if i >= height - 3 {
        return_val = false
    }
    else {
        return_val &= (full[i+1][j] == 'M');
        return_val &= (full[i+2][j] == 'A');
        return_val &= (full[i+3][j] == 'S');
    }

    return return_val;
}
fn left_found(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    if j <= 2 {
        return_val = false
    }
    else {
        return_val &= (full[i][j-1] == 'M');
        return_val &= (full[i][j-2] == 'A');
        return_val &= (full[i][j-3] == 'S');
    }

    return return_val;
}
fn up_found(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    if i <= 2 {
        return_val = false
    }
    else {
        return_val &= (full[i-1][j] == 'M');
        return_val &= (full[i-2][j] == 'A');
        return_val &= (full[i-3][j] == 'S');
    }

    return return_val;
}
fn right_found(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    if j >= width - 3 {
        return_val = false
    }
    else {
        return_val &= (full[i][j+1] == 'M');
        return_val &= (full[i][j+2] == 'A');
        return_val &= (full[i][j+3] == 'S');
    }

    return return_val;
}
fn bright_found(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    if i >= height - 3 || j >= width - 3 {
        return_val = false
    }
    else {
        return_val &= (full[i+1][j+1] == 'M');
        return_val &= (full[i+2][j+2] == 'A');
        return_val &= (full[i+3][j+3] == 'S');
    }

    return return_val;
}
fn uright_found(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    if i <= 2 || j >= width - 3 {
        return_val = false
    }
    else {
        return_val &= (full[i-1][j+1] == 'M');
        return_val &= (full[i-2][j+2] == 'A');
        return_val &= (full[i-3][j+3] == 'S');
    }

    return return_val;
}

fn uleft_found(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    if i <= 2 || j <= 2 {
        return_val = false
    }
    else {
        return_val &= (full[i-1][j-1] == 'M');
        return_val &= (full[i-2][j-2] == 'A');
        return_val &= (full[i-3][j-3] == 'S');
    }

    return return_val;
}

fn bleft_found(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    if i >= height - 3 || j <= 2 {
        return_val = false
    }
    else {
        return_val &= (full[i+1][j-1] == 'M');
        return_val &= (full[i+2][j-2] == 'A');
        return_val &= (full[i+3][j-3] == 'S');
    }

    return return_val;
}

fn aligned_cross(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    let vertical: bool;
    let horizontal: bool;
    if i >= height - 1 || j >= width - 1 || i <= 0 || j <= 0 {
        return_val = false;
    }
    else {
        vertical = ((full[i-1][j] == 'M') && (full[i+1][j] == 'S') || (full[i-1][j] == 'S') && (full[i+1][j] == 'M'));
        horizontal = ((full[i][j-1] == 'M') && (full[i][j+1] == 'S') || (full[i][j-1] == 'S') && (full[i][j+1] == 'M'));
        return_val &= vertical && horizontal;
    }
    return return_val;
}

fn aligned_diag(full: &Vec<Vec<char>>, i: usize, j: usize, width: usize, height: usize) -> bool {
    let mut return_val: bool = true;
    let up_right: bool;
    let down_right: bool;
    if i >= height - 1 || j >= width - 1 || i <= 0 || j <= 0 {
        return_val = false;
    }
    else {
        up_right = ((full[i-1][j-1] == 'M') && (full[i+1][j+1] == 'S') || (full[i-1][j-1] == 'S') && (full[i+1][j+1] == 'M'));
        down_right = ((full[i-1][j+1] == 'M') && (full[i+1][j-1] == 'S') || (full[i-1][j+1] == 'S') && (full[i+1][j-1] == 'M'));
        return_val &= up_right && down_right;
    }
    return return_val;
}

fn main() -> std::io::Result<()> {
    // Load the file
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let mut row: Vec<char> = Vec::new();
    // let mut full: Vec<&str> = Vec::new();
    let mut full: Vec<Vec<char>> = Vec::new();
    let lines = contents.lines();
    for l in lines {
        for c in l.chars() {
            row.push(c);
        }
        full.push(row.clone());
        row.clear();
    }
    let width: usize = full[0].len();
    let height: usize = full.len();
    let mut count: i32 = 0;
    let mut count_cross: i32 = 0;
    for (i, cline) in full.iter().enumerate() {
        for (j, _c) in cline.iter().enumerate() {
            if full[i][j] == 'A' {
                // if aligned_cross(&full, i, j, width, height){
                //     count_cross += 1;
                // }
                if aligned_diag(&full, i, j, width, height){
                    count_cross += 1;
                }
            }
            if full[i][j] == 'X' {
                if left_found(&full, i, j, width, height) {
                    count += 1;
                }
                if right_found(&full, i, j, width, height) {
                    count += 1;
                }
                if up_found(&full, i, j, width, height) {
                    count += 1;
                }
                if down_found(&full, i, j, width, height) {
                    count += 1;
                }
                if bright_found(&full, i, j, width, height) {
                    count += 1;
                }
                if uright_found(&full, i, j, width, height) {
                    count += 1;
                }
                if bleft_found(&full, i, j, width, height) {
                    count += 1;
                }
                if uleft_found(&full, i, j, width, height) {
                    count += 1;
                }
            }
        }
    }
    println!("Part 1 is {count}");
    println!("Part 2 is {count_cross}");
    Ok(())
}