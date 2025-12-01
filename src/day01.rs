use aoc2025::{AoCError, AoCResult, aoc_day};
use regex::Regex;

const EXAMPLE_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
R100
";

fn part1(input: &str) -> AoCResult {
    let steps = input.lines().filter(|l| !l.trim().is_empty());

    let re = Regex::new(r"^([LR])(\d+)$").expect("create regex");

    let mut zero_hits = 0;
    let mut current_rotation = 50;

    for step in steps {
        if let Some(caps) = re.captures(step) {
            let dir = caps.get(1).expect("get direction capture").as_str();
            let num: i32 = caps
                .get(2)
                .expect("get rotation capture")
                .as_str()
                .parse()
                .expect("parse rotation number");

            let rotation = match dir {
                "L" => -num,
                "R" => num,
                _ => panic!("unexpected direction: {dir}"),
            };

            current_rotation = (current_rotation + rotation) % 100;
            if current_rotation == 0 {
                zero_hits += 1
            }
        }
    }

    Ok(zero_hits.to_string())
}

fn part2(input: &str) -> AoCResult {
    let steps = input.lines().filter(|l| !l.trim().is_empty());

    let re = Regex::new(r"^([LR])(\d+)$").expect("create regex");

    let mut zero_hits = 0;
    let mut current_position = 50;

    for step in steps {
        if let Some(caps) = re.captures(step) {
            let dir = caps.get(1).expect("get direction capture").as_str();
            let num: i32 = caps
                .get(2)
                .expect("get rotation capture")
                .as_str()
                .parse()
                .expect("parse rotation number");

            let total_rotation = match dir {
                "L" => -num,
                "R" => num,
                _ => panic!("unexpected direction: {dir}"),
            };

            let actual_rotation = total_rotation % 100;
            let additional_passes = ((total_rotation - actual_rotation) / 100).abs();
            let before = zero_hits;

            println!("------------");
            println!("Starting from {current_position} and rotating {total_rotation}");
            println!(
                "In practice rotating by {actual_rotation} with {additional_passes} additional passes"
            );

            println!("------------");

            let theoretical_position = current_position + actual_rotation;
            let actual_position = if theoretical_position > 100 {
                theoretical_position % 100
            } else if theoretical_position == 100 {
                0
            } else if theoretical_position < 0 {
                100 - theoretical_position
            } else {
                theoretical_position
            };
            println!("Would theoretically rotate to {theoretical_position}");
            println!("Actually rotating to {actual_position}");

            println!("---- Summary ----");
            let total = zero_hits - before;
            println!(
                "Rotated by {total_rotation} from {current_position} to {actual_position} and made {total} hits"
            );

            println!("");
            current_position = actual_position;
        }
    }

    Ok(zero_hits.to_string())
}

// Use the extended macro form with `example = { ... }`:
aoc_day!(
    1,
    "Day 01",
    "Description";
    example = {
        label: "Example from problem statement",
        input: EXAMPLE_INPUT,
        expected1: Some(3),
        expected2: Some(7),
    }
);
