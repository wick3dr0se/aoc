use std::fs::read_to_string;
use regex::Regex;

fn get_mul_result(instructions: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    re.captures_iter(instructions)
        .map(|caps| {
            let x = caps[1].parse::<i32>().unwrap();
            let y = caps[2].parse::<i32>().unwrap();
            x * y
        })
        .sum::<i32>()
}

fn get_enabled_mul_result(instructions: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut mul_enabled = true;
    let mut enabled_sum: i32 = 0;
    let mut pos = 0;

    while let Some(caps) = re.find_at(&instructions, pos) {
        let matched_instr = &instructions[caps.start()..caps.end()];
        pos = caps.end();


        match matched_instr {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            _ => {
                if mul_enabled {
                    enabled_sum += get_mul_result(matched_instr);
                }
            }
        }
    }

    enabled_sum
}

fn main() {
    let input = read_to_string("inputs/day03.txt").unwrap();
    let mul_result = get_mul_result(&input);
    let enabled_mul_result = get_enabled_mul_result(&input);

    println!("Instruction multiplication result: {}", mul_result);
    println!("Enabled instruction multiplication result: {}", enabled_mul_result);
}
