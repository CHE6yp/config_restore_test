use clap::{Args, Parser};
use regex::Regex;

fn main() {
    let args = CRTArgs::parse();

    let sum = std::fs::read_to_string(args.destination)
        .unwrap()
        .lines()
        .map(|line| calculate_line(line) as usize)
        .sum::<usize>();

    println!("{}", sum);
}

fn calculate_line(line: &str) -> u8 {
    let pattern_first =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)").unwrap();

    let mut captures = pattern_first.captures_iter(line);

    let first_match = captures.next().unwrap().get(0).unwrap().as_str();
    let first = number_str_to_int(first_match);

    let last = match captures.last() {
        Some(capture) => number_str_to_int(capture.get(0).unwrap().as_str()),
        None => first,
    };

    return first * 10 + last;
}

fn number_str_to_int(string: &str) -> u8 {
    match string {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => unreachable!(),
    }
}

#[derive(Parser)]
struct CRTArgs {
    /// File destination
    destination: String,
}
