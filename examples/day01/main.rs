use std::{fs::File, io::{BufRead, BufReader}};

fn get_total_distance(col1: &mut [i32], col2: &mut [i32]) -> u32 {
    col1.sort();
    col2.sort();

    col1.iter().zip(col2)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn get_similarity_score(col1: &[i32], col2: &[i32]) -> i32 {
    let mut similarity_score = 0;

    for left in col1 {
        let matches = col2.iter().filter(|&right| right.eq(left)).count();
        similarity_score += matches as i32 * left;
    }

    return similarity_score
}

fn main() {
    let input = BufReader::new(File::open("examples/day01/input.txt").unwrap());
    let (mut left_col, mut right_col) = (Vec::new(), Vec::new());
    
    for buf in input.lines() {
        let line = buf.unwrap();
        let mut cols = line.split_whitespace();

        left_col.push(cols.next().unwrap().parse::<i32>().unwrap());
        right_col.push(cols.next().unwrap().parse::<i32>().unwrap());
    }

    let total_dist = get_total_distance(&mut left_col, &mut right_col);
    let similarity_score = get_similarity_score(&left_col, &right_col);

    println!("Total distance between list: {}", total_dist);
    println!("Similarity score: {}", similarity_score);
}