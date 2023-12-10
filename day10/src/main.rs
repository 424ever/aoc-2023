use core::panic;
use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};

use colored::Colorize;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq)]
struct Tile {
    dir1: Option<Direction>,
    dir2: Option<Direction>,
}

struct Map {
    tiles: Vec<String>,
}

fn part1(map: &Map) -> usize {
    map.find_loop().len() / 2
}

fn part2(map: &Map) -> usize {
    map.count_inside_loop()
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let map = Map::from(reader.lines().map(|l| l.unwrap()));

    println!("Part 1: {}", part1(&map).to_string().red());
    println!("Part 2: {}", part2(&map).to_string().red());

    Ok(())
}

impl Direction {
    fn back(&self) -> Direction {
        match self {
            Direction::North => Self::South,
            Direction::South => Self::North,
            Direction::East => Self::West,
            Direction::West => Self::East,
        }
    }
}

impl Tile {
    fn can_go(&self, dir: Direction) -> bool {
        self.go(dir).is_some()
    }

    fn go(&self, from: Direction) -> Option<Direction> {
        if Some(from) == self.dir1 {
            Some(self.dir2?.clone())
        } else if Some(from) == self.dir2 {
            Some(self.dir1?.clone())
        } else {
            None
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Direction::*;
        match value {
            '|' => Self {
                dir1: Some(North),
                dir2: Some(South),
            },
            '-' => Self {
                dir1: Some(West),
                dir2: Some(East),
            },
            'L' => Self {
                dir1: Some(North),
                dir2: Some(East),
            },
            'J' => Self {
                dir1: Some(North),
                dir2: Some(West),
            },
            '7' => Self {
                dir1: Some(South),
                dir2: Some(West),
            },
            'F' => Self {
                dir1: Some(South),
                dir2: Some(East),
            },
            '.' => Self {
                dir1: None,
                dir2: None,
            },
            'S' => Self {
                dir1: None,
                dir2: None,
            },
            _ => unreachable!("unknown char"),
        }
    }
}

impl Map {
    fn find_loop(&self) -> HashSet<(usize, usize)> {
        let mut dirs = HashSet::new();
        let mut x;
        let mut y;
        let mut dir;

        let start_idx = self.find_start().expect("No start found");
        (y, x) = start_idx;

        let dirs_from_start = self.directions_from_start();
        if dirs_from_start.len() != 2 {
            panic!("Not 2 directions from start: {:?}", dirs_from_start);
        }

        dir = dirs_from_start[0];

        loop {
            dirs.insert((y, x));
            /* apply dir */
            match dir {
                Direction::North => y -= 1,
                Direction::South => y += 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }

            if (y, x) == start_idx {
                break;
            }

            /* new dir */
            dir = self
                .tile_at(y, x)
                .expect("Attempted to go out of bounds")
                .go(dir.back())
                .expect("Attempted to go from a bad direction");
        }

        dirs
    }

    fn count_inside_loop(&self) -> usize {
        let found_loop = self.find_loop();
        let mut count = 0;

        for (y, line) in self.tiles.iter().enumerate() {
            let mut loop_intersect_count = 0;
            for (x, char) in line.char_indices() {
                let mut inside_loop = false;
                let on_loop = found_loop.contains(&(y, x));
                let mut tile = Tile::from(char);
                let mut counts_to_inter = false;

                if char == 'S' {
                    let start_dirs = self.directions_from_start();
                    tile = Tile {
                        dir1: Some(start_dirs[0]),
                        dir2: Some(start_dirs[1]),
                    }
                }

                if on_loop
                    && tile != Tile::from('-')
                    && tile != Tile::from('7')
                    && tile != Tile::from('F')
                {
                    loop_intersect_count += 1;
                    counts_to_inter = true;
                }

                if loop_intersect_count % 2 == 1 && !on_loop {
                    inside_loop = true;
                }

                if inside_loop {
                    count += 1;
                    print!("{}", char.to_string().green());
                } else if on_loop {
                    if counts_to_inter {
                        print!("{}", char.to_string().blue().underline());
                    } else {
                        print!("{}", char.to_string().blue());
                    }
                } else {
                    print!("{}", char.to_string().red());
                }
            }
            println!()
        }

        count
    }

    fn char_at(&self, y: usize, x: usize) -> Option<char> {
        let row = self.tiles.get(y)?;
        row.chars().nth(x)
    }

    fn tile_at(&self, y: usize, x: usize) -> Option<Tile> {
        Some(Tile::from(self.char_at(y, x)?))
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        for (i, line) in self.tiles.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == 'S' {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn directions_from_start(&self) -> Vec<Direction> {
        use Direction::*;

        let mut dirs = vec![];
        let (i, j) = self.find_start().expect("No start found");

        if i > 0 && self.tile_at(i - 1, j).is_some_and(|t| t.can_go(South)) {
            dirs.push(North);
        }
        if self.tile_at(i + 1, j).is_some_and(|t| t.can_go(North)) {
            dirs.push(South);
        }
        if j > 0 && self.tile_at(i, j - 1).is_some_and(|t| t.can_go(East)) {
            dirs.push(West);
        }
        if self.tile_at(i, j + 1).is_some_and(|t| t.can_go(West)) {
            dirs.push(West);
        }

        dirs
    }
}

impl<I> From<I> for Map
where
    I: Iterator<Item = String>,
{
    fn from(value: I) -> Self {
        Self {
            tiles: value.collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Map};

    #[test]
    fn test_part1_1() {
        let map = Map::from(
            vec![".....", ".S-7.", ".|.|.", ".L-J.", "....."]
                .iter()
                .map(|l| l.to_string()),
        );
        assert_eq!(part1(&map), 4);
    }

    #[test]
    fn test_part1_2() {
        let map = Map::from(
            vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."]
                .iter()
                .map(|l| l.to_string()),
        );
        assert_eq!(part1(&map), 8);
    }

    #[test]
    fn test_part2_1() {
        let map = Map::from(
            vec![
                "...........",
                ".S-------7.",
                ".|F-----7|.",
                ".||.....||.",
                ".||.....||.",
                ".|L-7.F-J|.",
                ".|..|.|..|.",
                ".L--J.L--J.",
                "...........",
            ]
            .iter()
            .map(|l| l.to_string()),
        );
        assert_eq!(part2(&map), 4);
    }

    #[test]
    fn test_part2_2() {
        let map = Map::from(
            vec![
                "..........",
                ".S------7.",
                ".|F----7|.",
                ".||....||.",
                ".||....||.",
                ".|L-7F-J|.",
                ".|..||..|.",
                ".L--JL--J.",
                "..........",
            ]
            .iter()
            .map(|l| l.to_string()),
        );
        assert_eq!(part2(&map), 4);
    }

    #[test]
    fn test_part2_3() {
        let map = Map::from(
            vec![
                ".F----7F7F7F7F-7....",
                ".|F--7||||||||FJ....",
                ".||.FJ||||||||L7....",
                "FJL7L7LJLJ||LJ.L-7..",
                "L--J.L7...LJS7F-7L7.",
                "....F-J..F7FJ|L7L7L7",
                "....L7.F7||L7|.L7L7|",
                ".....|FJLJ|FJ|F7|.LJ",
                "....FJL-7.||.||||...",
                "....L---J.LJ.LJLJ...",
            ]
            .iter()
            .map(|l| l.to_string()),
        );
        assert_eq!(part2(&map), 8);
    }

    #[test]
    fn test_part2_4() {
        let map = Map::from(
            vec![
                "FF7FSF7F7F7F7F7F---7",
                "L|LJ||||||||||||F--J",
                "FL-7LJLJ||||||LJL-77",
                "F--JF--7||LJLJ7F7FJ-",
                "L---JF-JLJ.||-FJLJJ7",
                "|F|F-JF---7F7-L7L|7|",
                "|FFJF7L7F-JF7|JL---7",
                "7-L-JL7||F7|L7F-7F7|",
                "L.L7LFJ|||||FJL7||LJ",
                "L7JLJL-JLJLJL--JLJ.L",
            ]
            .iter()
            .map(|l| l.to_string()),
        );
        assert_eq!(part2(&map), 10);
    }
}
