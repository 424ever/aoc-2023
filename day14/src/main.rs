use std::{collections::HashMap, env, fs::read_to_string, slice::Chunks};

struct Position {
    content: Vec<char>,
    width: usize,
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = read_to_string(filename).unwrap();
    let mut pos = Position::from_input(&input).unwrap();

    println!("Part 1: {}", pos.slide_north().load_north());
    pos.cycle(1_000_000_000);
    println!("Part 2: {}", pos.load_north());
}

fn rows<T>(v: &Vec<T>, width: usize) -> Chunks<'_, T> {
    v.chunks(width)
}

impl Position {
    fn from_input(input: &str) -> Option<Position> {
        let content = input
            .lines()
            .map(|l| Vec::from_iter(l.chars()))
            .flatten()
            .collect();
        let width = input.lines().last()?.len();
        for l in input.lines() {
            if l.len() != width {
                return None;
            }
        }
        Some(Self { content, width })
    }

    fn height(&self) -> usize {
        self.content.len() / self.width
    }

    fn slide_north(&self) -> Position {
        fn at(v: &Vec<char>, w: usize, r: usize, c: usize) -> Option<&char> {
            v.get(r * w + c)
        }

        fn set_at(v: &mut Vec<char>, w: usize, r: usize, c: usize, val: char) -> Option<()> {
            *v.get_mut(r * w + c).unwrap() = val;
            Some(())
        }

        fn new_row(v: &Vec<char>, w: usize, row_north_of_start: usize, c: usize) -> usize {
            for i in (0..=row_north_of_start).rev() {
                if *at(v, w, i, c).unwrap() != '.' {
                    return i + 1;
                }
            }
            return 0;
        }

        fn move_rock(v: &mut Vec<char>, w: usize, r: usize, c: usize) {
            if r == 0 {
                return;
            }

            let nr = new_row(v, w, r - 1, c);
            set_at(v, w, r, c, '.').unwrap();
            set_at(v, w, nr, c, 'O').unwrap();
        }

        let mut content = self.content.clone();

        for r in 1..self.height() {
            for c in 0..self.width {
                if at(&content, self.width, r, c) == Some(&'O') {
                    move_rock(&mut content, self.width, r, c);
                }
            }
        }

        Self {
            content,
            width: self.width,
        }
    }

    fn cycle(&mut self, count: usize) -> Option<()> {
        fn at(v: &Vec<char>, w: usize, r: usize, c: usize) -> Option<&char> {
            v.get(r * w + c)
        }

        fn at_mut(v: &mut Vec<char>, w: usize, r: usize, c: usize) -> Option<&mut char> {
            v.get_mut(r * w + c)
        }

        // https://stackoverflow.com/a/35438327
        fn rotate(v: &mut Vec<char>, w: usize) -> Option<()> {
            for l in 0..w / 2 {
                let first = l;
                let last = w - first - 1;
                for e in first..last {
                    let o = e - first;

                    let top = *at(v, w, first, e)?;
                    let right = *at(v, w, e, last)?;
                    let bottom = *at(v, w, last, last - o)?;
                    let left = *at(v, w, last - o, first)?;

                    *at_mut(v, w, first, e)? = left;
                    *at_mut(v, w, e, last)? = top;
                    *at_mut(v, w, last, last - o)? = right;
                    *at_mut(v, w, last - o, first)? = bottom;
                }
            }

            Some(())
        }

        fn new_row(v: &Vec<char>, w: usize, row_north_of_start: usize, c: usize) -> Option<usize> {
            for i in (0..=row_north_of_start).rev() {
                if *at(v, w, i, c)? != '.' {
                    return Some(i + 1);
                }
            }
            return Some(0);
        }

        fn move_rock(v: &mut Vec<char>, w: usize, r: usize, c: usize) -> Option<()> {
            if r == 0 {
                return Some(());
            }

            let nr = new_row(v, w, r - 1, c)?;
            *at_mut(v, w, r, c)? = '.';
            *at_mut(v, w, nr, c)? = 'O';

            Some(())
        }

        fn move_all_round(v: &mut Vec<char>, w: usize) -> Option<()> {
            for r in 0..w {
                for c in 0..w {
                    if *at(v, w, r, c)? == 'O' {
                        move_rock(v, w, r, c)?;
                    }
                }
            }

            Some(())
        }

        fn do_cycle(v: &mut Vec<char>, w: usize) -> Option<()> {
            move_all_round(v, w)?;
            rotate(v, w)?;
            move_all_round(v, w)?;
            rotate(v, w)?;
            move_all_round(v, w)?;
            rotate(v, w)?;
            move_all_round(v, w)?;
            rotate(v, w)?;

            Some(())
        }

        if self.height() != self.width {
            return None;
        }

        let mut seen = HashMap::new();
        for i in 1..=count {
            do_cycle(&mut self.content, self.width);

            match seen.get(&self.content) {
                Some(o) => {
                    let cycle = i - o;
                    let rem = (count - i) % cycle;
                    for _ in 0..rem {
                        do_cycle(&mut self.content, self.width);
                    }
                    break;
                }
                None => {
                    seen.insert(self.content.clone(), i);
                }
            };
        }

        Some(())
    }

    fn load_north(&self) -> usize {
        fn count_round_in_row(row: &[char]) -> usize {
            row.iter().filter(|c| **c == 'O').count()
        }

        rows(&self.content, self.width)
            .map(count_round_in_row)
            .enumerate()
            .map(|(i, c)| (self.height() - i) * c)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Position;

    #[test]
    fn test_part1() {
        let input = concat!(
            "O....#....\n",
            "O.OO#....#\n",
            ".....##...\n",
            "OO.#O....O\n",
            ".O.....O#.\n",
            "O.#..O.#.#\n",
            "..O..#O..O\n",
            ".......O..\n",
            "#....###..\n",
            "#OO..#...."
        );
        let pos = Position::from_input(input).unwrap().slide_north();
        assert_eq!(pos.load_north(), 136);
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            "O....#....\n",
            "O.OO#....#\n",
            ".....##...\n",
            "OO.#O....O\n",
            ".O.....O#.\n",
            "O.#..O.#.#\n",
            "..O..#O..O\n",
            ".......O..\n",
            "#....###..\n",
            "#OO..#...."
        );
        let mut pos = Position::from_input(input).unwrap();
        pos.cycle(1_000_000_000).unwrap();
        assert_eq!(pos.load_north(), 64);
    }
}
