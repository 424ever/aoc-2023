use std::{
    collections::HashMap,
    env,
    fmt::{Display, Write},
    fs::read_to_string,
    io::ErrorKind,
    iter::repeat,
    str::FromStr,
};

use colored::Colorize;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(PartialEq, Eq)]
struct Row {
    springs: Vec<Condition>,
    damaged: Vec<usize>,
}

fn solve(rows: &Vec<Row>) -> usize {
    rows.iter().map(|r| r.arrangements()).sum()
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let input = read_to_string(filename)?;
    let rows: Vec<_> = input.lines().map(|l| Row::from_str(l).unwrap()).collect();
    let unfolded = rows.iter().map(|r| r.unfolded()).collect();

    println!("Part 1: {}", solve(&rows).to_string().green());
    println!("Part 2: {}", solve(&unfolded).to_string().green());

    Ok(())
}

impl Row {
    fn arrangements(&self) -> usize {
        let mut cache = HashMap::new();

        fn can_fit_damaged_streak(springs: &[Condition], count: usize) -> bool {
            // Very much inspired by to https://github.com/Sp00ph
            let mut springs = springs;
            for _ in 0..count {
                if let [Condition::Unknown | Condition::Damaged, rest @ ..] = springs {
                    springs = rest;
                } else {
                    return false;
                }
            }

            if springs.first() == Some(&Condition::Damaged) {
                return false;
            } else {
                return true;
            }
        }

        fn insert_cache(
            cache: &mut HashMap<(usize, usize), usize>,
            key: (usize, usize),
            val: usize,
        ) -> usize {
            cache.insert(key, val);
            return val;
        }

        fn count<'a>(
            springs: &[Condition],
            damaged: &[usize],
            cache: &'a mut HashMap<(usize, usize), usize>,
        ) -> usize {
            let key = (springs.len(), damaged.len());

            if let Some(val) = cache.get(&key) {
                return *val;
            }

            if springs.is_empty() {
                return insert_cache(cache, key, usize::from(damaged.is_empty()));
            }

            if damaged.is_empty() {
                let res = usize::from(!springs.iter().any(|c| matches!(c, Condition::Damaged)));
                return insert_cache(cache, key, res);
            }

            match springs[0] {
                Condition::Operational => {
                    let res = count(&springs[1..], damaged, cache);
                    return insert_cache(cache, key, res);
                }
                Condition::Damaged => {
                    if can_fit_damaged_streak(springs, damaged[0]) {
                        let res = count(
                            &springs.get(damaged[0] + 1..).unwrap_or(&[]),
                            &damaged[1..],
                            cache,
                        );
                        return insert_cache(cache, key, res);
                    } else {
                        return insert_cache(cache, key, 0);
                    }
                }
                Condition::Unknown => {
                    let operational_count = count(&springs[1..], damaged, cache);
                    let mut damaged_count = 0;
                    if can_fit_damaged_streak(springs, damaged[0]) {
                        damaged_count = count(
                            &springs.get(damaged[0] + 1..).unwrap_or(&[]),
                            &damaged[1..],
                            cache,
                        );
                    }
                    let res = operational_count + damaged_count;
                    return insert_cache(cache, key, res);
                }
            }
        }

        count(&self.springs, &self.damaged, &mut cache)
    }

    fn unfolded(&self) -> Self {
        let mut springs = Vec::with_capacity(self.springs.len() * 5);

        for _ in 0..5 {
            springs.append(&mut self.springs.iter().cloned().collect());
            springs.push(Condition::Unknown);
        }
        springs.pop();

        Self {
            springs,
            damaged: repeat(self.damaged.iter())
                .take(5)
                .flatten()
                .copied()
                .collect(),
        }
    }
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Row {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let springs = s
            .split_whitespace()
            .nth(0)
            .ok_or(())?
            .chars()
            .map(|c| Condition::from(c))
            .collect();
        let damaged = s
            .split_whitespace()
            .nth(1)
            .ok_or(())?
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Self { springs, damaged })
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .springs
                .iter()
                .map(|c| match c {
                    Condition::Operational => '.',
                    Condition::Damaged => '#',
                    Condition::Unknown => '?',
                })
                .join(""),
        )?;
        f.write_char(' ')?;
        f.write_str(&self.damaged.iter().join(","))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{solve, Row};

    #[test]
    fn test_part1_line1() {
        let input = "???.### 1,1,3";
        assert_eq!(Row::from_str(&input).unwrap().arrangements(), 1);
    }

    #[test]
    fn test_part1_line2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(Row::from_str(&input).unwrap().arrangements(), 4);
    }

    #[test]
    fn test_part1_line3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(Row::from_str(&input).unwrap().arrangements(), 1);
    }

    #[test]
    fn test_part1_line4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(Row::from_str(&input).unwrap().arrangements(), 1);
    }

    #[test]
    fn test_part1_line5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(Row::from_str(&input).unwrap().arrangements(), 4);
    }

    #[test]
    fn test_part1_line6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(Row::from_str(&input).unwrap().arrangements(), 10);
    }

    #[test]
    fn test_part1() {
        let input = concat!(
            "???.### 1,1,3\n",
            ".??..??...?##. 1,1,3\n",
            "?#?#?#?#?#?#?#? 1,3,1,6\n",
            "????.#...#... 4,1,1\n",
            "????.######..#####. 1,6,5\n",
            "?###???????? 3,2,1\n",
        );
        let rows = input.lines().map(|l| Row::from_str(l).unwrap()).collect();
        assert_eq!(solve(&rows), 21);
    }

    #[test]
    fn test_unfolded() {
        let input = ".# 1";
        let row = Row::from_str(input).unwrap();
        let unfolded = row.unfolded();

        assert_eq!(row.to_string(), ".# 1");
        assert_eq!(unfolded.to_string(), ".#?.#?.#?.#?.# 1,1,1,1,1");
    }

    #[test]
    fn test_part2_line1() {
        let input = "???.### 1,1,3";
        assert_eq!(Row::from_str(&input).unwrap().unfolded().arrangements(), 1);
    }

    #[test]
    fn test_part2_line2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(
            Row::from_str(&input).unwrap().unfolded().arrangements(),
            16384
        );
    }

    #[test]
    fn test_part2_line3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(Row::from_str(&input).unwrap().unfolded().arrangements(), 1);
    }

    #[test]
    fn test_part2_line4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(Row::from_str(&input).unwrap().unfolded().arrangements(), 16);
    }

    #[test]
    fn test_part2_line5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(
            Row::from_str(&input).unwrap().unfolded().arrangements(),
            2500
        );
    }

    #[test]
    fn test_part2_line6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(
            Row::from_str(&input).unwrap().unfolded().arrangements(),
            506250
        );
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            "???.### 1,1,3\n",
            ".??..??...?##. 1,1,3\n",
            "?#?#?#?#?#?#?#? 1,3,1,6\n",
            "????.#...#... 4,1,1\n",
            "????.######..#####. 1,6,5\n",
            "?###???????? 3,2,1\n",
        );
        let rows = input
            .lines()
            .map(|l| Row::from_str(l).unwrap().unfolded())
            .collect();
        assert_eq!(solve(&rows), 525152);
    }
}
