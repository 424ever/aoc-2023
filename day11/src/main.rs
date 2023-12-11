use std::{env, fs::read_to_string, io::ErrorKind};

use colored::Colorize;
use itertools::Itertools;

use core::panic;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Universe {
    tiles: Vec<Vec<Tile>>,
    width: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Galaxy {
    number: usize,
    coord: Coord,
}

struct Galaxies<'a> {
    u: &'a Universe,
    index: usize,
    next_number: usize,
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    Galaxy,
    RepeatetEmpty(usize),
}

fn solve(u: &Universe) -> usize {
    let galaxies: Vec<_> = u.galaxies().collect();
    galaxies
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

impl<'a> Iterator for Galaxies<'a> {
    type Item = Galaxy;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = None;

        while let Some((ch, canskip)) = self.u.tile_at_index(self.index) {
            self.index += canskip;
            if ch == '#' {
                res = Some(Galaxy {
                    number: self.next_number,
                    coord: self.u.index_to_coord(self.index - 1),
                });
                self.next_number += 1;
                break;
            }
        }

        res
    }
}

impl Universe {
    fn new(input: String) -> Universe {
        let mut tiles = vec![];
        let width = input.lines().nth(0).expect("No lines").len();

        for line in input.lines() {
            let mut row = vec![];
            if line.len() != width {
                panic!("Different line lengths");
            }
            for ch in line.chars() {
                if ch != '.' && ch != '#' {
                    panic!("Unexpected char");
                }
                if ch == '.' {
                    row.push(Tile::Empty);
                } else if ch == '#' {
                    row.push(Tile::Galaxy);
                }
            }
            tiles.push(row);
        }

        Self { tiles, width }
    }

    fn expanded(&self, replace_empty_with: usize) -> Universe {
        let mut tiles = Vec::with_capacity(self.tiles.capacity());
        let mut width = self.width();

        for (idx, _) in self.tiles.iter().enumerate() {
            let coord = self.index_to_coord(idx);
            if self.is_col_empty(coord.col) {
                if coord.row == 0 {
                    width += replace_empty_with - 1;
                }
            }
        }

        for (row, r) in self.tiles.iter().enumerate() {
            let mut rowvec = vec![];
            if self.is_row_empty(row) {
                rowvec.push(Tile::RepeatetEmpty(width));
                for _ in 0..replace_empty_with {
                    tiles.push(rowvec.clone());
                }
            } else {
                for (col, t) in r.iter().enumerate() {
                    if self.is_col_empty(col) {
                        rowvec.push(Tile::RepeatetEmpty(replace_empty_with));
                    } else {
                        rowvec.push(t.clone());
                    }
                }
                tiles.push(rowvec);
            }
        }

        Self { tiles, width }
    }

    fn galaxies(&self) -> Galaxies {
        Galaxies {
            u: self,
            index: 0,
            next_number: 1,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        Coord {
            row: index / self.width(),
            col: index % self.width(),
        }
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn tile_at(&self, coord: &Coord) -> Option<(char, usize)> {
        let mut idx = 0;
        for t in self.tiles.get(coord.row)? {
            let clen = match t {
                Tile::Empty => 1,
                Tile::Galaxy => 1,
                Tile::RepeatetEmpty(s) => *s,
            };

            if idx + clen > coord.col {
                return Some((t.repr(), clen));
            }
            idx += clen;
        }
        return None;
    }

    fn tile_at_xy(&self, row: usize, col: usize) -> Option<char> {
        Some(self.tile_at(&Coord { row, col })?.0)
    }

    fn is_row_empty(&self, row: usize) -> bool {
        for i in 0..self.width() {
            if self.tile_at_xy(row, i).unwrap() == '#' {
                return false;
            }
        }
        return true;
    }

    fn is_col_empty(&self, col: usize) -> bool {
        for row in 0..self.height() {
            if self.tile_at_xy(row, col).unwrap() == '#' {
                return false;
            }
        }
        return true;
    }

    fn tile_at_index(&self, index: usize) -> Option<(char, usize)> {
        self.tile_at(&self.index_to_coord(index))
    }
}

impl Coord {
    fn manhattan_dist(&self, other: &Self) -> usize {
        self.col.abs_diff(other.col) + self.row.abs_diff(other.row)
    }
}

impl Tile {
    fn repr(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Galaxy => '#',
            Tile::RepeatetEmpty(_) => '.',
        }
    }
}

#[cfg(test)]
mod tests {
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
            u.galaxies().collect::<Vec<_>>(),
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
