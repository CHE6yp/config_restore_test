use regex::Regex;

fn main() {
    let example = "aaaa4ddd3";

    println!("{:?}", calculate_line(example));
}

fn calculate_line(line: &str) -> u8 {
    let pattern_first =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)").unwrap();
    let pattern_last =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)$").unwrap();

    let match_first = pattern_first
        .captures(line)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();
    let match_last = pattern_last
        .captures(line)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();

    let first = number_str_to_int(match_first);
    let last = number_str_to_int(match_last);

    first * 10 + last
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
