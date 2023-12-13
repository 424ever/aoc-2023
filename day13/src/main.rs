use std::{env, fs::read_to_string, io::ErrorKind, usize};

struct Pattern {
    rows: Vec<char>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum Reflection {
    Horizontal { rows: usize },
    Vertical { cols: usize },
}

fn part1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    lines
        .split(|l| l.is_empty())
        .map(|ls| Pattern::try_from(ls).unwrap())
        .map(|p| p.reflection_without_change())
        .map(|r| r.value())
        .sum()
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    lines
        .split(|l| l.is_empty())
        .map(|ls| Pattern::try_from(ls).unwrap())
        .map(|p| p.reflection_with_single_change())
        .map(|r| r.value())
        .sum()
}

fn difference_between_strs(s1: &str, s2: &str) -> usize {
    assert!(s1.len() == s2.len());
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| *c1 != *c2)
        .count()
}

fn difference_between_str_list(v1: &[String], v2: &[String]) -> usize {
    v1.iter()
        .zip(v2.iter())
        .map(|(s1, s2)| difference_between_strs(s1, s2))
        .sum()
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let input = read_to_string(filename)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

impl Reflection {
    fn value(&self) -> usize {
        match self {
            Reflection::Horizontal { rows } => 100 * rows,
            Reflection::Vertical { cols } => *cols,
        }
    }
}

impl Pattern {
    fn reflection_without_change(&self) -> Reflection {
        assert!(self.height > 1);
        assert!(self.width > 1);
        // horizontal
        for r in 1..self.height {
            let mut above: Vec<_> = self.rows().take(r).collect();
            above.reverse();
            let below: Vec<_> = self.rows().skip(r).collect();
            if above.iter().zip(below.iter()).all(|(a, b)| a == b) {
                return Reflection::Horizontal { rows: r };
            }
        }

        // vertical
        for c in 1..self.width {
            let mut left: Vec<_> = self.cols().take(c).collect();
            left.reverse();
            let right: Vec<_> = self.cols().skip(c).collect();
            if left.iter().zip(right.iter()).all(|(l, r)| l == r) {
                return Reflection::Vertical { cols: c };
            }
        }

        unreachable!()
    }

    fn reflection_with_single_change(&self) -> Reflection {
        assert!(self.height > 1);
        assert!(self.width > 1);
        // horizontal
        for r in 1..self.height {
            let mut above: Vec<_> = self.rows().take(r).collect();
            above.reverse();
            let below: Vec<_> = self.rows().skip(r).collect();

            if difference_between_str_list(above.as_slice(), below.as_slice()) == 1 {
                return Reflection::Horizontal { rows: r };
            }
        }

        // vertical
        for c in 1..self.width {
            let mut left: Vec<_> = self.cols().take(c).collect();
            left.reverse();
            let right: Vec<_> = self.cols().skip(c).collect();

            if difference_between_str_list(left.as_slice(), right.as_slice()) == 1 {
                return Reflection::Vertical { cols: c };
            }
        }

        unreachable!()
    }

    fn rows(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new((0..self.height).map(|r| self.row(r).unwrap()))
    }

    fn row(&self, r: usize) -> Option<String> {
        Some(
            self.rows
                .get(r * self.width..(r + 1) * self.width)?
                .iter()
                .collect(),
        )
    }

    fn cols(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new((0..self.width).map(|c| self.rows().map(|r| r.chars().nth(c).unwrap()).collect()))
    }
}

impl<'a> TryFrom<&'a [&str]> for Pattern {
    type Error = ();
    fn try_from(value: &'a [&str]) -> Result<Self, Self::Error> {
        let mut rows = vec![];
        let width = value.first().ok_or(())?.len();

        for row in value {
            if width != row.len() {
                return Err(());
            }
            rows.extend(row.chars());
        }

        Ok(Self {
            rows,
            width,
            height: value.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Pattern, Reflection};

    #[test]
    fn test_part1_p1() {
        let input = vec![
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];
        assert_eq!(
            Pattern::try_from(input.as_slice())
                .unwrap()
                .reflection_without_change(),
            Reflection::Vertical { cols: 5 }
        );
    }

    #[test]
    fn test_part1_p2() {
        let input = vec![
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ];
        assert_eq!(
            Pattern::try_from(input.as_slice())
                .unwrap()
                .reflection_without_change(),
            Reflection::Horizontal { rows: 4 }
        );
    }

    #[test]
    fn test_part1_belowlonger() {
        let input = vec![
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
            "#...##..#",
            "..#####..",
        ];
        assert_eq!(
            Pattern::try_from(input.as_slice())
                .unwrap()
                .reflection_without_change(),
            Reflection::Horizontal { rows: 4 }
        );
    }

    #[test]
    fn test_part1() {
        let input = concat!(
            "#.##..##.\n",
            "..#.##.#.\n",
            "##......#\n",
            "##......#\n",
            "..#.##.#.\n",
            "..##..##.\n",
            "#.#.##.#.\n",
            "\n",
            "#...##..#\n",
            "#....#..#\n",
            "..##..###\n",
            "#####.##.\n",
            "#####.##.\n",
            "..##..###\n",
            "#....#..#\n",
        );
        assert_eq!(part1(input), 405);
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            "#.##..##.\n",
            "..#.##.#.\n",
            "##......#\n",
            "##......#\n",
            "..#.##.#.\n",
            "..##..##.\n",
            "#.#.##.#.\n",
            "\n",
            "#...##..#\n",
            "#....#..#\n",
            "..##..###\n",
            "#####.##.\n",
            "#####.##.\n",
            "..##..###\n",
            "#....#..#\n",
        );
        assert_eq!(part2(input), 400);
    }
}
