use std::{
    collections::HashSet,
    env,
    fs::read_to_string,
    hash::{Hash, Hasher},
    io::ErrorKind,
};

use colored::Colorize;
use itertools::Itertools;

use core::panic;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Universe {
    galaxies: HashSet<Galaxy>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Galaxy {
    number: usize,
    coord: Coord,
}

fn solve(u: &Universe) -> usize {
    u.galaxies
        .iter()
        .tuple_combinations()
        .map(|(c1, c2)| c1.coord.manhattan_dist(&c2.coord))
        .sum()
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let input = read_to_string(filename)?;
    let universe = Universe::new(input);

    println!("Part 1: {}", solve(&universe.expanded(2)).to_string().red());
    println!(
        "Part 2: {}",
        solve(&universe.expanded(1_000_000)).to_string().red()
    );
    Ok(())
}

impl Universe {
    fn new(input: String) -> Universe {
        let mut galaxies = HashSet::new();
        let width = input.lines().nth(0).expect("No lines").len();
        let mut galaxy_num: usize = 1;

        for (row, line) in input.lines().enumerate() {
            if line.len() != width {
                panic!("Different line lengths");
            }
            for (col, ch) in line.char_indices() {
                if ch != '.' && ch != '#' {
                    panic!("Unexpected char");
                }
                if ch == '#' {
                    galaxies.insert(Galaxy {
                        number: galaxy_num,
                        coord: Coord { row, col },
                    });
                    galaxy_num += 1;
                }
            }
        }

        Self {
            galaxies,
            width,
            height: input.lines().count(),
        }
    }

    fn expanded(&self, replace_empty_with: usize) -> Universe {
        let empty_rows: Vec<_> = (0..self.height())
            .filter(|r| self.is_row_empty(*r))
            .collect();
        let empty_cols: Vec<_> = (0..self.width())
            .filter(|c| self.is_col_empty(*c))
            .collect();

        fn count_empty_rows_below(empty_rows: &Vec<usize>, row: usize) -> usize {
            empty_rows.iter().filter(|r| **r < row).count()
        }

        fn count_empty_cols_before(empty_cols: &Vec<usize>, col: usize) -> usize {
            empty_cols.iter().filter(|c| **c < col).count()
        }

        fn get_with_replacement(old: usize, count: usize, replacement: usize) -> usize {
            (old - count) + (count * replacement)
        }

        let galaxies = self
            .galaxies
            .iter()
            .map(|g| {
                let empty_rows = count_empty_rows_below(&empty_rows, g.coord.row);
                let empty_cols = count_empty_cols_before(&empty_cols, g.coord.col);

                let row = get_with_replacement(g.coord.row, empty_rows, replace_empty_with);
                let col = get_with_replacement(g.coord.col, empty_cols, replace_empty_with);

                Galaxy {
                    number: g.number,
                    coord: Coord { row, col },
                }
            })
            .collect();

        Self {
            galaxies,
            width: get_with_replacement(self.width(), empty_cols.len(), replace_empty_with),
            height: get_with_replacement(self.height(), empty_rows.len(), replace_empty_with),
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn is_row_empty(&self, row: usize) -> bool {
        for g in self.galaxies.iter() {
            if g.coord.row == row {
                return false;
            }
        }
        return true;
    }

    fn is_col_empty(&self, col: usize) -> bool {
        for g in self.galaxies.iter() {
            if g.coord.col == col {
                return false;
            }
        }
        return true;
    }
}

impl Coord {
    fn manhattan_dist(&self, other: &Self) -> usize {
        self.col.abs_diff(other.col) + self.row.abs_diff(other.row)
    }
}

impl Hash for Galaxy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{solve, Coord, Galaxy, Universe};

    #[test]
    fn test_expand_1() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        let u = Universe::new(input.to_string());
        assert_eq!(u.width(), 10);
        assert_eq!(u.height(), 10);
        let u = u.expanded(2);
        assert_eq!(u.width(), 13);
        assert_eq!(u.height(), 12);
    }

    #[test]
    fn test_expand_10() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        let u = Universe::new(input.to_string());
        assert_eq!(u.width(), 10);
        assert_eq!(u.height(), 10);
        let u = u.expanded(10);
        assert_eq!(u.width(), 37);
        assert_eq!(u.height(), 28);
    }

    #[test]
    fn test_galaxies() {
        let input = concat!("...#.\n", ".#...\n", "#....\n",);
        let u = Universe::new(input.to_string());
        assert_eq!(
            u.galaxies,
            HashSet::from_iter(
                vec![
                    Galaxy {
                        coord: Coord { row: 0, col: 3 },
                        number: 1
                    },
                    Galaxy {
                        coord: Coord { row: 1, col: 1 },
                        number: 2
                    },
                    Galaxy {
                        coord: Coord { row: 2, col: 0 },
                        number: 3
                    },
                ]
                .iter()
                .cloned()
            )
        )
    }

    #[test]
    fn test_distance() {
        let c1 = Coord { row: 6, col: 1 };
        let c2 = Coord { row: 11, col: 5 };

        assert_eq!(c1.manhattan_dist(&c2), 9);
    }

    #[test]
    fn test_part1() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        let u = Universe::new(input.to_string());
        assert_eq!(solve(&u.expanded(2)), 374);
    }

    #[test]
    fn test_part2_1() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        let u = Universe::new(input.to_string());
        assert_eq!(solve(&u.expanded(10)), 1030);
    }

    #[test]
    fn test_part2_2() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        let u = Universe::new(input.to_string());
        assert_eq!(solve(&u.expanded(100)), 8410);
    }
}
