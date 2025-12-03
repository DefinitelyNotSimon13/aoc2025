use aoc2025::{AoCResult, aoc_day, input_lines};

const EXAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn append_nums(nums: &[u32]) -> u64 {
    nums.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

fn get_joltage(batteries: &[u32], active_banks_amount: usize) -> u64 {
    let mut active_banks: Vec<u32> = Vec::with_capacity(active_banks_amount);

    batteries.into_iter().for_each(|&new_bat| {
        if active_banks.len() < active_banks_amount {
            active_banks.push(new_bat);
            return;
        }

        // Option 1 dont take
        let keep_sum = append_nums(&active_banks);

        // Option 2 replace one and append new
        let mut replace = active_banks.clone();
        let to_replace = replace.windows(2).position(|w| w[1] > w[0]);

        if let Some(to_replace) = to_replace {
            replace.remove(to_replace);
        } else {
            replace.pop();
        }

        replace.push(new_bat);
        let replace_sum = append_nums(&replace);

        if replace_sum > keep_sum {
            active_banks = replace;
        }
    });

    append_nums(&active_banks)
}

fn get_battery_bank(line: &str) -> Vec<u32> {
    line.chars()
        .map(|c| c.to_digit(10).expect("parse num from char"))
        .collect()
}

fn part1(input: &str) -> AoCResult<u64> {
    Ok(input_lines(input)
        .map(get_battery_bank)
        .map(|b| get_joltage(&b, 2))
        .sum())
}

fn part2(input: &str) -> AoCResult<u64> {
    Ok(input_lines(input)
        .map(get_battery_bank)
        .map(|b| get_joltage(&b, 12))
        .sum())
}

aoc_day!(
    3,
    "Lobby",
    example = {
        label: "Example",
        input: EXAMPLE_INPUT,
        expected1: Some(357),
        expected2: Some(3121910778619),
    }
);
