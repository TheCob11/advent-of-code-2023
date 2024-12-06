use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Card<const JOKERS: bool = false> {
    Ace,
    King,
    Queen,
    JackOrJoker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl<const JOKERS: bool> PartialOrd<Self> for Card<JOKERS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const JOKERS: bool> Ord for Card<JOKERS> {
    fn cmp(&self, other: &Self) -> Ordering {
        if JOKERS {
            let a = if let Card::JackOrJoker = self {
                13
            } else {
                *self as u8
            };
            let b = if let Card::JackOrJoker = other {
                13
            } else {
                *other as u8
            };
            a.cmp(&b)
        } else {
            (*self as u8).cmp(&(*other as u8))
        }
    }
}

impl<const JOKERS: bool> Display for Card<JOKERS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char((*self).into())
    }
}

impl<const JOKERS: bool> From<Card<JOKERS>> for char {
    fn from(value: Card<JOKERS>) -> Self {
        match value {
            Card::Ace => 'A',
            Card::King => 'K',
            Card::Queen => 'Q',
            Card::JackOrJoker => 'J',
            Card::Ten => 'T',
            Card::Nine => '9',
            Card::Eight => '8',
            Card::Seven => '7',
            Card::Six => '6',
            Card::Five => '5',
            Card::Four => '4',
            Card::Three => '3',
            Card::Two => '2',
        }
    }
}

impl<const JOKERS: bool> TryFrom<char> for Card<JOKERS> {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use anyhow::anyhow;
        use Card as C;
        Ok(match value {
            'A' => C::Ace,
            'K' => C::King,
            'Q' => C::Queen,
            'J' => C::JackOrJoker,
            'T' => C::Ten,
            '9' => C::Nine,
            '8' => C::Eight,
            '7' => C::Seven,
            '6' => C::Six,
            '5' => C::Five,
            '4' => C::Four,
            '3' => C::Three,
            '2' => C::Two,
            '1' | '0' => Err(anyhow!("invalid digit"))?,
            _ => Err(anyhow!("invalid card"))?,
        })
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Hand<const JOKERS: bool> {
    hand_type: HandType,
    cards: [Card<JOKERS>; 5],
}

impl<const JOKERS: bool> TryFrom<[Card<JOKERS>; 5]> for Hand<JOKERS> {
    type Error = anyhow::Error;
    fn try_from(cards: [Card<JOKERS>; 5]) -> Result<Self, Self::Error> {
        Ok(Self {
            hand_type: cards.into(),
            cards,
        })
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl<const JOKERS: bool> From<[Card<JOKERS>; 5]> for HandType {
    fn from(cards: [Card<JOKERS>; 5]) -> Self {
        let mut card_counts = [0usize; 13];
        for card in cards {
            let count = card_counts
                .get_mut(card as usize)
                .expect("should be in range");
            if !JOKERS && 4.eq(count) {
                return Self::FiveOfAKind;
            }
            *count += 1;
        }
        let jokers = if JOKERS {
            std::mem::take(&mut card_counts[Card::<JOKERS>::JackOrJoker as usize])
        } else {
            0
        };
        card_counts.sort_unstable();
        let [.., next_max_count, max_count] = card_counts;
        match (max_count + jokers, next_max_count) {
            (5.., _) => Self::FiveOfAKind,
            (4, _) => Self::FourOfAKind,
            (3, 2) => Self::FullHouse,
            (3, 1) => Self::ThreeOfAKind,
            (2, 2) => Self::TwoPair,
            (2, 1) => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Bid<const JOKERS: bool>(Hand<JOKERS>, usize);

impl<const JOKERS: bool> FromStr for Bid<JOKERS> {
    type Err = anyhow::Error;
    // Expects hand space bet like "32T3K 765"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bet_str) = s.split_at(5);
        let cards: [Card<JOKERS>; 5] = hand_str
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| anyhow::anyhow!("shouldn't change length or anything"))?;
        Ok(Self(Hand::try_from(cards)?, bet_str[1..].parse()?))
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut bids: Vec<Bid<false>> = input
        .lines()
        .map(str::parse)
        .map(|x| x.expect("should be valid bid"))
        .collect();
    bids.sort_unstable();
    bids.reverse();
    bids.iter().enumerate().map(|(i, x)| x.1 * (i + 1)).sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let mut bids: Vec<Bid<true>> = input
        .lines()
        .map(str::parse)
        .map(|x| x.expect("should be valid bid"))
        .collect();
    bids.sort_unstable();
    bids.reverse();
    bids.iter().enumerate().map(|(i, x)| x.1 * (i + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 5905);
    }
}
