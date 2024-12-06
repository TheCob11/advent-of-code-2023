#[aoc(day1, part1, slice_pattern)]
pub fn solve_part1_slice_pattern(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            u32::from(
                match line.trim_matches(|x: char| !x.is_ascii_digit()).as_bytes() {
                    [x] => (x - b'0') * 11,
                    [x, .., y] => (x - b'0') * 10 + (y - b'0'),
                    _ => panic!("something went wrong"),
                },
            )
        })
        .sum()
}

#[aoc(day1, part1, str_pattern)]
pub fn solve_part1_str_pattern(input: &str) -> u32 {
    const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    input
        .lines()
        .map(|line| {
            let mut digit_chars = line.matches(DIGIT_CHARS);
            let tens_digit = (digit_chars.next().unwrap().as_bytes()[0] as char)
                .to_digit(10)
                .expect("should be num");
            let ones_digit = digit_chars.last().map_or(tens_digit, |x| {
                (x.as_bytes()[0] as char)
                    .to_digit(10)
                    .expect("should be num")
            });
            tens_digit * 10 + ones_digit
        })
        .sum()
}

#[aoc(day1, part1, regex)]
pub fn solve_part1_regex(input: &str) -> u32 {
    use regex::Regex;
    let re = Regex::new(r"(?m)^.*?([0-9]).*?([0-9])?[^0-9]*$").expect("should be valid regex");
    re.captures_iter(input)
        .map(|caps| {
            let caps = caps;
            // println!("{caps:?}");
            let tens_digit = (caps[1].as_bytes()[0] as char)
                .to_digit(10)
                .expect("should be num");
            let ones_digit = caps.get(2).map_or(tens_digit, |x| {
                (x.as_str().as_bytes()[0] as char)
                    .to_digit(10)
                    .expect("should be num")
            });
            tens_digit * 10 + ones_digit
        })
        .sum()
}

#[aoc(day1, part2, regex)]
pub fn solve_part2_regex(input: &str) -> u32 {
    use fancy_regex::Regex;
    let re = Regex::new(
        r"(?m)^.*?([0-9]|one|two|three|four|five|six|seven|eight|nine)(?:.*([0-9]|one|two|three|four|five|six|seven|eight|nine))?.*(?![0-9]|one|two|three|four|five|six|seven|eight|nine)$"
    )
        .expect("should be valid regex");
    re.captures_iter(input)
        .map(|caps| {
            let caps = caps.unwrap();
            let tens_digit = get_digit(&caps[1]);
            let ones_digit = caps.get(2).map_or(tens_digit, |x| get_digit(x.as_str()));
            tens_digit * 10 + ones_digit
        })
        .sum()
}

fn get_digit(s: &str) -> u32 {
    match s.as_bytes() {
        b"zero" | b"0" => 0,
        b"one" | b"1" => 1,
        b"two" | b"2" => 2,
        b"three" | b"3" => 3,
        b"four" | b"4" => 4,
        b"five" | b"5" => 5,
        b"six" | b"6" => 6,
        b"seven" | b"7" => 7,
        b"eight" | b"8" => 8,
        b"nine" | b"9" => 9,
        _ => panic!("should be num"),
    }
}
