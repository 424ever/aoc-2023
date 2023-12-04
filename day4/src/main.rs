use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::BufReader,
    io::{BufRead, ErrorKind},
    str::FromStr,
};

struct Card {
    id: u32,
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!(
        "{}\n{}",
        part1.ok_or(ErrorKind::Other)?,
        part2.ok_or(ErrorKind::Other)?
    );
    Ok(())
}

fn set_from_string(s: &str) -> HashSet<u32> {
    s.trim()
        .split(' ')
        .map(|s| s.parse())
        .filter_map(|r| r.ok())
        .collect()
}

fn part1(lines: &Vec<String>) -> Option<u32> {
    let cards: Result<Vec<Card>, _> = lines.iter().map(|l| Card::from_str(l.as_str())).collect();
    let cards = cards.ok()?;
    Some(
        cards
            .iter()
            .map(|c| c.count())
            .filter(|c| c > &0)
            .map(|c| 2u32.pow(c as u32 - 1))
            .sum(),
    )
}

fn part2(lines: &Vec<String>) -> Option<u32> {
    let cards: Result<Vec<Card>, _> = lines.iter().map(|l| Card::from_str(l.as_str())).collect();
    let cards = cards.ok()?;
    let mut cardcounts: HashMap<u32, u32> = cards.iter().map(|c| (c.id, 1)).collect();
    for card in cards {
        let currcount = cardcounts.get(&card.id).unwrap().to_owned();
        for i in 0..card.count() {
            let key: u32 = card.id + i + 1;
            if let Some(x) = cardcounts.get_mut(&key) {
                *x += currcount;
            }
        }
    }
    Some(cardcounts.values().sum())
}

impl Card {
    fn count(&self) -> u32 {
        self.numbers.intersection(&self.winning_numbers).count() as u32
    }
}

enum CardErr {
    Error,
}
impl FromStr for Card {
    type Err = CardErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, numbers) = s.split_once(':').ok_or(Self::Err::Error)?;
        let (numbers, winning_numbers) = numbers.split_once('|').ok_or(Self::Err::Error)?;
        let numbers = set_from_string(numbers);
        let winning_numbers = set_from_string(winning_numbers);
        let id = id
            .split_once(' ')
            .ok_or(Self::Err::Error)?
            .1
            .trim()
            .parse()
            .unwrap();
        Ok(Card {
            id,
            numbers,
            winning_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let lines = lines.iter().map(|s| s.to_string()).collect();
        assert_eq!(part1(&lines).unwrap(), 13);
    }

    #[test]
    fn test_part_2() {
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let lines = lines.iter().map(|s| s.to_string()).collect();
        assert_eq!(part2(&lines).unwrap(), 30);
    }
}
