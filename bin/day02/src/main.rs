use std::{fs::File, io::{BufRead, BufReader}};

enum Direction {
    Increasing,
    Decreasing
}

fn is_safe_report(report: &[i32], skip_index: Option<usize>) -> bool {
    let mut const_change = None;

    report.iter()
        .enumerate()
        .filter(|&(i, _)| Some(i) != skip_index)
        .map(|(_, &lvl)| lvl)
        .collect::<Vec<_>>()
        .windows(2)
        .all(|lvl| {
            let diff = lvl[0].abs_diff(lvl[1]);
            if diff < 1 || diff > 3 {
                return false;
            }

            if lvl[0] < lvl[1] {
                if let Some(Direction::Decreasing) = const_change {
                    return false;
                }

                const_change = Some(Direction::Increasing);
            } else if lvl[0] > lvl[1] {
                if let Some(Direction::Increasing) = const_change {
                    return false;
                }

                const_change = Some(Direction::Decreasing);
            }
            true
        })
}

fn get_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter()
        .filter(|lvls| is_safe_report(lvls, None))
        .count()
}

fn get_dampened_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter()
        .filter(|lvls| is_safe_report(lvls, None) || (0..lvls.len()).any(|i| {
            is_safe_report(lvls, Some(i))
        })).count()
}

fn main() {
    let input = BufReader::new(File::open("inputs/day02.txt").unwrap());
    let reports: Vec<Vec<i32>> = input.lines()
        .map(|buf| {
            buf.unwrap()
                .split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let safe_reports = get_safe_reports(&reports);
    let dampened_safe_reports = get_dampened_safe_reports(&reports);

    println!("Safe reports: {}", safe_reports);
    println!("Dampened safe reports: {}", dampened_safe_reports);
}