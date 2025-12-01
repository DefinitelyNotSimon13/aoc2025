use aoc2025::{AoCResult, aoc_day};
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

            // IT HURTS SOOO MUCH :(
            for i in 0..total_rotation.abs() {
                if total_rotation > 0 {
                    current_position += 1;
                    if current_position == 100 {
                        zero_hits += 1;
                        current_position = 0;
                    }
                } else {
                    current_position -= 1;
                    if current_position == 0 {
                        zero_hits += 1;
                    }
                    if current_position == -1 {
                        current_position = 99;
                    }
                }
            }
        }
    }

    Ok(zero_hits.to_string())
}

aoc_day!(
    1,
    "Secret Entrance",
    example = {
        label: "Example",
        input: EXAMPLE_INPUT,
        expected1: Some(3),
        expected2: Some(6),
    }
);
