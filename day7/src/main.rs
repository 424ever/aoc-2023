use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    JJoker,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    JNoJoker,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, Debug)]
struct Bid {
    cards: [Card; 5],
    amount: u32,
    with_joker: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let bids_without_joker: Vec<Bid> = lines
        .iter()
        .map(|l| Bid::try_from(&l, false).unwrap())
        .collect();
    let bids_with_joker: Vec<Bid> = lines
        .iter()
        .map(|l| Bid::try_from(&l, true).unwrap())
        .collect();
    println!("{}", solve(&bids_without_joker).unwrap());
    println!("{}", solve(&bids_with_joker).unwrap());
    Ok(())
}

fn solve(bids: &Vec<Bid>) -> Option<u32> {
    Some(
        bids.iter()
            .sorted()
            .enumerate()
            .map(|(i, bid)| (i as u32 + 1) * bid.amount)
            .sum(),
    )
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        }

        if self.kind() < other.kind() {
            return Ordering::Less;
        }

        if self.kind() > other.kind() {
            return Ordering::Greater;
        }

        for i in 0..5 {
            if self.cards[i] > other.cards[i] {
                return Ordering::Greater;
            }

            if self.cards[i] < other.cards[i] {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

impl Bid {
    fn sequences(&self) -> Vec<(&Card, usize)> {
        let mut groups: Vec<(&Card, usize)> = self
            .cards
            .iter()
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(ge0, group)| (ge0, group.count()))
            .collect();

        if self.with_joker {
            groups.sort_by(|g1, g2| -> std::cmp::Ordering {
                let mut order = g1.1.cmp(&g2.1);
                // only report J as the longest if there is no tie
                if g1.0 == &Card::JJoker {
                    order = Ordering::Less;
                } else if g2.0 == &Card::JJoker {
                    order = Ordering::Greater;
                }
                return order;
            });

            let mut j_group_size = 0;
            let j_group = groups.iter().find(|g| g.0 == &Card::JJoker);
            if let Some(j_group) = j_group {
                j_group_size = j_group.1;
            }

            let longest_group = groups.last_mut().unwrap();
            if longest_group.0 != &Card::JJoker {
                longest_group.1 += j_group_size;
                groups.retain(|g| g.0 != &Card::JJoker);
            }
        }

        return groups;
    }

    pub fn kind(&self) -> HandType {
        let sequences = self.sequences();
        let mut hand_type = HandType::HighCard;

        if sequences.len() == 1 {
            hand_type = HandType::FiveOfAKind;
        } else if sequences.len() == 2 {
            if sequences.iter().any(|(_, c)| *c == 4) {
                hand_type = HandType::FourOfAKind;
            } else {
                hand_type = HandType::FullHouse;
            }
        } else if sequences.len() == 3 {
            if sequences.iter().any(|(_, c)| *c == 3) {
                hand_type = HandType::ThreeOfAKind;
            } else {
                hand_type = HandType::TwoPair;
            }
        } else if sequences.len() == 4 {
            hand_type = HandType::OnePair;
        }

        return hand_type;
    }

    fn try_from(s: &str, with_joker: bool) -> Result<Self, ()> {
        let cards: [Card; 5] = s[..5]
            .chars()
            .map(|c| Card::try_from(c, with_joker).unwrap())
            .collect::<Vec<Card>>()
            .try_into()
            .or(Err(()))?;
        let amount: u32 = s.split_once(' ').ok_or(())?.1.parse().or(Err(()))?;
        Ok(Bid {
            cards,
            amount,
            with_joker,
        })
    }
}

impl Card {
    fn try_from(value: char, with_joker: bool) -> Result<Self, ()> {
        match value {
            'A' => Ok(Self::A),
            'K' => Ok(Self::K),
            'Q' => Ok(Self::Q),
            'J' => {
                if with_joker {
                    Ok(Self::JJoker)
                } else {
                    Ok(Self::JNoJoker)
                }
            }
            'T' => Ok(Self::T),
            '9' => Ok(Self::N9),
            '8' => Ok(Self::N8),
            '7' => Ok(Self::N7),
            '6' => Ok(Self::N6),
            '5' => Ok(Self::N5),
            '4' => Ok(Self::N4),
            '3' => Ok(Self::N3),
            '2' => Ok(Self::N2),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{solve, Bid, HandType};

    #[test]
    fn test_part_1() {
        let bids = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|l| Bid::try_from(l, false).unwrap())
        .collect();
        assert_eq!(solve(&bids).unwrap(), 6440);
    }

    #[test]
    fn test_part_2() {
        let bids = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|l| Bid::try_from(l, true).unwrap())
        .collect();
        assert_eq!(solve(&bids).unwrap(), 5905);
    }

    #[test]
    fn test_joker_type() {
        let bid = Bid::try_from("KJJJK 500", true).unwrap();
        assert_eq!(bid.kind(), HandType::FiveOfAKind);
    }
}
