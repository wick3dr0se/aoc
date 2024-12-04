use std::{fs::File, io::{BufRead, BufReader}};

fn check_xmas(grid: &[Vec<char>], rows: usize, cols: usize) -> usize {
    let mut count = 0;

    for idx in 0..(rows * cols) {
        let (i, j) = (idx / cols, idx % cols);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if (0..4 as isize).all(|k| {
                    let x = (i as isize + k * dx) as usize;
                    let y = (j as isize + k * dy) as usize;

                    x < rows && y < cols &&
                        grid[x][y] == "XMAS".chars().nth(k as usize).unwrap()
                }) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_x_mas(grid: &[Vec<char>], rows: usize, cols: usize) -> usize {
    let mut count = 0;

    for idx in 0..(rows * cols) {
        let (i, j) = (idx / cols, idx % cols);

        if i > 0 && j > 0 && i < rows - 1 && j < cols - 1 && grid[i][j] == 'A' {
            let pat = (
                grid[i - 1][j - 1], grid[i + 1][j + 1],
                grid[i - 1][j + 1], grid[i + 1][j - 1]
            );

            if matches!(pat,
                ('M', 'S', 'M', 'S') | ('S', 'M', 'S', 'M') |
                ('M', 'S', 'S', 'M') | ('S', 'M', 'M', 'S'))
            {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = BufReader::new(File::open("inputs/day04.txt").unwrap());
    let grid: Vec<Vec<char>> = input.lines().map(|line| {
        line.unwrap().chars().collect()
    }).collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    let xmas_count = check_xmas(&grid, rows, cols);
    let x_mas_count = check_x_mas(&grid, rows, cols);

    println!("XMAS appears {} times", xmas_count);
    println!("X-MAS appears {} times", x_mas_count);
}