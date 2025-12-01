use console::{Term, style};
use std::fs;
use std::time::{Duration, Instant};
use thiserror::Error;

pub type AoCResult = Result<String, AoCError>;

#[derive(Debug, Error)]
pub enum AoCError {
    #[error("not implemented yet")]
    NotImplemented,

    #[error("{0}")]
    Custom(String),
}

impl AoCError {
    pub fn custom(msg: impl Into<String>) -> Self {
        AoCError::Custom(msg.into())
    }
}

pub struct Example {
    pub label: String,
    pub input: String,
    pub expected1: Option<String>,
    pub expected2: Option<String>,
}

pub fn run_day<F1, F2>(
    day: u8,
    title: &str,
    description: &str,
    example: Option<Example>,
    part1: F1,
    part2: F2,
) where
    F1: Fn(&str) -> AoCResult,
    F2: Fn(&str) -> AoCResult,
{
    let term = Term::stdout();

    let header = format!(
        "{} {} {}",
        // EMOJIS IM CODE :O
        style("🎄 Advent of Code 2025").green().bold(),
        style("—").dim(),
        style(format!("Day {:02}: {}", day, title)).cyan().bold()
    );

    let url = format!("https://adventofcode.com/2025/day/{}", day);
    let separator = style("─".repeat(header.len().min(80))).dim().to_string();

    let desc_title = style("❄️ Description").yellow().bold();
    let link_title = style("🔗 Puzzle").blue().bold();

    let header_text =
        format!("{header}\n{separator}\n{desc_title}\n{description}\n\n{link_title}\n{url}\n",);
    let _ = term.write_line(&header_text);

    let example_path = format!("inputs/day{:02}_example.txt", day);
    let input_path = format!("inputs/day{:02}.txt", day);

    fn load(path: &str) -> Option<String> {
        match fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    let example = if let Some(ex) = example {
        Some(ex)
    } else if let Some(example_input) = load(&example_path) {
        Some(Example {
            label: format!("Example input from {example_path}"),
            input: example_input,
            expected1: None,
            expected2: None,
        })
    } else {
        None
    };

    let input = load(&input_path)
        .unwrap_or_else(|| panic!("Could not read real input file `{input_path}`"));

    if let Some(ref ex) = example {
        let _ = term.write_line(&style(format!("> {} — Part 1", ex.label)).bold().to_string());
        let (res, t) = timed(&part1, &ex.input);
        print_result(&term, "Example Part 1", &res, ex.expected1.clone(), t);
        let _ = term.write_line("");
    }

    let _ = term.write_line(
        &style(format!("> Running Part 1 on real input: {input_path}"))
            .bold()
            .to_string(),
    );
    let (ans1, t1) = timed(&part1, &input);
    print_result(&term, "Part 1", &ans1, None, t1);
    let _ = term.write_line("");

    if let Some(ref ex) = example {
        let _ = term.write_line(&style(format!("> {} — Part 2", ex.label)).bold().to_string());
        let (res, t) = timed(&part2, &ex.input);
        print_result(&term, "Example Part 2", &res, ex.expected2.clone(), t);
        let _ = term.write_line("");
    }

    let _ = term.write_line(
        &style(format!("> Running Part 2 on real input: {input_path}"))
            .bold()
            .to_string(),
    );
    let (ans2, t2) = timed(&part2, &input);
    print_result(&term, "Part 2", &ans2, None, t2);

    let _ = term.write_line(&style("Good luck & merry coding! 🎁").magenta().to_string());
}

pub fn run_example_literal<F1, F2, E1, E2>(
    label: &str,
    input: &str,
    expected1: Option<E1>,
    expected2: Option<E2>,
    part1: F1,
    part2: F2,
) where
    F1: Fn(&str) -> AoCResult,
    F2: Fn(&str) -> AoCResult,
    E1: ToString,
    E2: ToString,
{
    let term = Term::stdout();
    let _ = term.write_line(&style(format!("> {label}")).bold().to_string());

    let (res1, t1) = timed(&part1, input);
    let (res2, t2) = timed(&part2, input);

    let exp1_str = expected1.map(|e| e.to_string());
    let exp2_str = expected2.map(|e| e.to_string());

    print_result(&term, "Example Part 1", &res1, exp1_str, t1);
    print_result(&term, "Example Part 2", &res2, exp2_str, t2);

    let _ = term.write_line("");
}

fn timed<F>(f: &F, input: &str) -> (AoCResult, Duration)
where
    F: Fn(&str) -> AoCResult,
{
    let start = Instant::now();
    let result = f(input);
    (result, start.elapsed())
}

fn print_result(
    term: &Term,
    label: &str,
    result: &AoCResult,
    expected: Option<String>,
    dur: Duration,
) {
    let time_str = format!("{:.3?}", dur);

    let line = match result {
        Ok(got) => match expected {
            Some(ref exp) if exp == got => format!(
                "{} {} {} ({})",
                style(label).green().bold(),
                style("✓").green(),
                style(got).bold(),
                style(time_str).dim(),
            ),
            Some(ref exp) => format!(
                "{} {} got={} expected={} ({})",
                style(label).red().bold(),
                style("✗").red(),
                style(got).bold(),
                style(exp).yellow(),
                style(time_str).dim(),
            ),
            None => format!(
                "{} {} ({})",
                style(label).cyan().bold(),
                style(got).bold(),
                style(time_str).dim(),
            ),
        },
        Err(AoCError::NotImplemented) => {
            format!(
                "{} {} ({})",
                style(label).dim(),
                style("skipped – not implemented yet").dim(),
                style(time_str).dim(),
            )
        }
        Err(err) => format!(
            "{} {} ({})",
            style(label).red().bold(),
            style(format!("ERROR: {err}")).red(),
            style(time_str).dim(),
        ),
    };

    let _ = term.write_line(&line);
}

#[macro_export]
macro_rules! aoc_day {
    (
        $day:literal,
        $title:expr,
        $description:expr
    ) => {
        fn main() {
            aoc2025::run_day($day, $title, $description, None, crate::part1, crate::part2);
        }
    };

    (
        $day:literal,
        $title:expr,
        $description:expr;
        example = {
            label: $label:expr,
            input: $input:expr,
            expected1: $expected1:expr,
            expected2: $expected2:expr $(,)?
        }
    ) => {
        fn main() {
            let example = aoc2025::Example {
                label: $label.to_string(),
                input: $input.to_string(),
                expected1: $expected1.map(|e| e.to_string()),
                expected2: $expected2.map(|e| e.to_string()),
            };

            aoc2025::run_day(
                $day,
                $title,
                $description,
                Some(example),
                crate::part1,
                crate::part2,
            );
        }
    };
}
