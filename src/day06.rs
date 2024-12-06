#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::if_not_else
)]
fn win_margin(time: usize, record: usize) -> usize {
    let half_time = time as f64 / 2.0;
    let discriminant = (half_time * half_time - record as f64).sqrt();
    let perfect_square_offset: i8 = if discriminant.trunc() != discriminant {
        1
    } else {
        -1
    };
    (((half_time + discriminant).floor() - (half_time - discriminant).ceil()) as usize)
        .saturating_add_signed(perfect_square_offset as isize)
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (times, dists) = input.split_once('\n').expect("should have a newline");
    times[10..]
        .split_whitespace()
        .zip(dists[10..].split_whitespace())
        .map(|(time, dist)| {
            (
                time.parse().expect("should be num"),
                dist.parse().expect("should be num"),
            )
        })
        .map(|(time, dist)| win_margin(time, dist))
        .product()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (time, dist) = input.split_once('\n').expect("should have a newline");
    win_margin(
        time[10..]
            .replace(' ', "")
            .parse()
            .expect("should be valid nums"),
        dist[10..]
            .replace(' ', "")
            .parse()
            .expect("should be valid nums"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 71503);
    }
}
