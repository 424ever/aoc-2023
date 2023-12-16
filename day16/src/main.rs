use std::{collections::HashSet, env, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    coord: Coord,
    dir: char,
}

fn coord_to_index(c: &Coord, w: usize) -> usize {
    c.row * w + c.col
}

fn at(field: &Vec<char>, w: usize, c: &Coord) -> Option<char> {
    field.get(coord_to_index(c, w)).cloned()
}

fn can_go_up(c: &Coord) -> bool {
    c.row > 0
}

fn can_go_down(c: &Coord, h: usize) -> bool {
    c.row < h - 1
}

fn can_go_left(c: &Coord) -> bool {
    c.col > 0
}

fn can_go_right(c: &Coord, w: usize) -> bool {
    c.col < w - 1
}

fn handle_beam(beam: &Beam, tile: char, newbeams: &mut Vec<Beam>, width: usize, height: usize) {
    match beam.dir {
        'u' => {
            if tile == '.' || tile == '|' {
                if can_go_up(&beam.coord) {
                    newbeams.push(beam.go_up());
                }
            } else if tile == '-' {
                newbeams.push(beam.left());
                newbeams.push(beam.right());
            } else if tile == '/' {
                if can_go_right(&beam.coord, width) {
                    newbeams.push(beam.go_right());
                }
            } else if tile == '\\' {
                if can_go_left(&beam.coord) {
                    newbeams.push(beam.go_left());
                }
            }
        }
        'd' => {
            if tile == '.' || tile == '|' {
                if can_go_down(&beam.coord, height) {
                    newbeams.push(beam.go_down());
                }
            } else if tile == '-' {
                newbeams.push(beam.left());
                newbeams.push(beam.right());
            } else if tile == '/' {
                if can_go_left(&beam.coord) {
                    newbeams.push(beam.go_left());
                }
            } else if tile == '\\' {
                if can_go_right(&beam.coord, width) {
                    newbeams.push(beam.go_right());
                }
            }
        }
        'l' => {
            if tile == '.' || tile == '-' {
                if can_go_left(&beam.coord) {
                    newbeams.push(beam.go_left());
                }
            } else if tile == '|' {
                newbeams.push(beam.up());
                newbeams.push(beam.down());
            } else if tile == '/' && can_go_down(&beam.coord, height) {
                newbeams.push(beam.go_down());
            } else if tile == '\\' && can_go_up(&beam.coord) {
                newbeams.push(beam.go_up());
            }
        }
        'r' => {
            if tile == '.' || tile == '-' {
                if can_go_right(&beam.coord, width) {
                    newbeams.push(beam.go_right());
                }
            } else if tile == '|' {
                newbeams.push(beam.up());
                newbeams.push(beam.down());
            } else if tile == '/' && can_go_up(&beam.coord) {
                newbeams.push(beam.go_up());
            } else if tile == '\\' && can_go_down(&beam.coord, height) {
                newbeams.push(beam.go_down());
            }
        }
        _ => unreachable!(),
    }
}

fn simulate(field: &Vec<char>, width: usize, height: usize, initial_beam: Beam) -> usize {
    let mut energized = HashSet::new();
    let mut beams = vec![];
    beams.push(initial_beam);

    while !beams.is_empty() {
        let mut newbeams = vec![];

        for beam in &beams {
            if !energized.contains(beam) {
                energized.insert(beam.clone());
                let tile = at(&field, width, &beam.coord).unwrap();
                handle_beam(&beam, tile, &mut newbeams, width, height);
            }
        }

        beams.clear();
        beams.append(&mut newbeams);
    }

    energized
        .iter()
        .map(move |b| b.coord)
        .collect::<HashSet<_>>()
        .len()
}

fn part1(input: &str) -> usize {
    let width = input.lines().last().unwrap().len();
    let height = input.lines().count();
    let mut field = vec![];
    for line in input.lines() {
        if line.len() != width {
            panic!("Different line lengths");
        }
        field.extend(line.chars());
    }

    simulate(
        &field,
        width,
        height,
        Beam {
            dir: 'r',
            coord: Coord { row: 0, col: 0 },
        },
    )
}

fn part2(input: &str) -> usize {
    let width = input.lines().last().unwrap().len();
    let height = input.lines().count();
    let mut field = vec![];
    for line in input.lines() {
        if line.len() != width {
            panic!("Different line lengths");
        }
        field.extend(line.chars());
    }

    let mut res = 0;

    /* top & bottom */
    for i in 0..width {
        res = res.max(simulate(
            &field,
            width,
            height,
            Beam {
                coord: Coord { row: 0, col: i },
                dir: 'd',
            },
        ));
        res = res.max(simulate(
            &field,
            width,
            height,
            Beam {
                coord: Coord {
                    row: height - 1,
                    col: i,
                },
                dir: 'u',
            },
        ));
    }
    /* left & right */
    for i in 0..height {
        res = res.max(simulate(
            &field,
            width,
            height,
            Beam {
                coord: Coord { row: i, col: 0 },
                dir: 'r',
            },
        ));
        res = res.max(simulate(
            &field,
            width,
            height,
            Beam {
                coord: Coord {
                    row: i,
                    col: width - 1,
                },
                dir: 'l',
            },
        ));
    }

    res
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = read_to_string(filename).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

impl Coord {
    fn up(&self) -> Coord {
        Coord {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }
}

impl Beam {
    fn up(&self) -> Beam {
        Beam {
            coord: self.coord.clone(),
            dir: 'u',
        }
    }

    fn go_up(&self) -> Beam {
        Beam {
            coord: self.coord.up(),
            dir: 'u',
        }
    }

    fn down(&self) -> Beam {
        Beam {
            coord: self.coord.clone(),
            dir: 'd',
        }
    }

    fn go_down(&self) -> Beam {
        Beam {
            coord: self.coord.down(),
            dir: 'd',
        }
    }

    fn left(&self) -> Beam {
        Beam {
            coord: self.coord.clone(),
            dir: 'l',
        }
    }

    fn go_left(&self) -> Beam {
        Beam {
            coord: self.coord.left(),
            dir: 'l',
        }
    }

    fn right(&self) -> Beam {
        Beam {
            coord: self.coord.clone(),
            dir: 'r',
        }
    }

    fn go_right(&self) -> Beam {
        Beam {
            coord: self.coord.right(),
            dir: 'r',
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = concat!(
            ".|...\\....\n",
            "|.-.\\.....\n",
            ".....|-...\n",
            "........|.\n",
            "..........\n",
            ".........\\\n",
            "..../.\\\\..\n",
            ".-.-/..|..\n",
            ".|....-|.\\\n",
            "..//.|....\n",
        );
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn test_part2() {
        let input = concat!(
            ".|...\\....\n",
            "|.-.\\.....\n",
            ".....|-...\n",
            "........|.\n",
            "..........\n",
            ".........\\\n",
            "..../.\\\\..\n",
            ".-.-/..|..\n",
            ".|....-|.\\\n",
            "..//.|....\n",
        );
        assert_eq!(part2(input), 51);
    }
}
