use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind},
    str::FromStr,
};

struct Sequence {
    nums: Vec<i64>,
}

fn part1(sequences: &Vec<Sequence>) -> i64 {
    sequences.iter().map(|s| s.next()).sum()
}

fn part2(sequences: &Vec<Sequence>) -> i64 {
    sequences.iter().map(|s| s.prev()).sum()
}

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let sequences = lines.map(|l| Sequence::from_str(&l).unwrap()).collect();

    println!("{}", part1(&sequences));
    println!("{}", part2(&sequences));

    Ok(())
}

impl Sequence {
    fn sequence_of_diffs(&self) -> Sequence {
        let mut nums: Vec<i64> = vec![];

        for i in 1..self.nums.len() {
            let low = self.nums.get(i - 1).unwrap();
            let high = self.nums.get(i).unwrap();

            nums.push(high - low);
        }

        Self { nums }
    }

    fn is_identity(&self) -> bool {
        self.nums.iter().all(|i| *i == 0)
    }

    fn next(&self) -> i64 {
        if self.is_identity() {
            0
        } else {
            self.nums.last().expect("no numbers in sequence") + self.sequence_of_diffs().next()
        }
    }

    fn prev(&self) -> i64 {
        if self.is_identity() {
            0
        } else {
            self.nums.first().expect("no numbers in sequence") - self.sequence_of_diffs().prev()
        }
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_ascii_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();

        Ok(Self { nums })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{part1, part2, Sequence};

    #[test]
    fn test_part1() {
        let seqs = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"]
            .iter()
            .map(|l| Sequence::from_str(l).unwrap())
            .collect();
        assert_eq!(part1(&seqs), 114);
    }

    #[test]
    fn test_part2() {
        let seqs = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"]
            .iter()
            .map(|l| Sequence::from_str(l).unwrap())
            .collect();
        assert_eq!(part2(&seqs), 2);
    }
}
