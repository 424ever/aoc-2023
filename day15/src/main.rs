use std::{env, fs::read_to_string};

#[derive(Debug)]
struct Step {
    label: String,
    op: char,
    focal: u64,
}

struct Lens {
    label: String,
    focal: u64,
}

fn holliday_hash(input: &str) -> u8 {
    let mut state: u32 = 0;
    for b in input.as_bytes() {
        state += *b as u32;
        state *= 17;
        state %= 256;
    }
    state as u8
}

fn parse_steps(input: &str) -> Option<Vec<Step>> {
    let mut insns = vec![];
    for insn in input.split(',') {
        let label = insn
            .chars()
            .take_while(|c| c.is_ascii_alphabetic())
            .collect::<String>();
        let op = insn
            .chars()
            .skip_while(|c| c.is_ascii_alphabetic())
            .next()?;
        if op != '-' && op != '=' {
            return None;
        }
        let focal = if op == '=' {
            insn.chars()
                .skip_while(|c| c.is_ascii_alphabetic())
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .ok()?
        } else {
            0
        };
        insns.push(Step { label, op, focal });
    }
    Some(insns)
}

fn part1(input: &str) -> u64 {
    input.split(',').map(holliday_hash).map(|h| h as u64).sum()
}

fn part2(input: &str) -> Option<u64> {
    let steps = parse_steps(input)?;
    let mut map: Vec<Vec<Lens>> = Vec::with_capacity(256);
    for _ in 0..256 {
        map.push(vec![]);
    }

    for step in steps {
        let idx = holliday_hash(&step.label);
        let v = map.get_mut(idx as usize).unwrap();

        let i = v.iter().position(|l| l.label == step.label);

        match step.op {
            '-' => {
                if let Some(i) = i {
                    v.remove(i);
                }
            }
            '=' => {
                if let Some(i) = i {
                    v.get_mut(i).unwrap().focal = step.focal;
                } else {
                    v.push(Lens {
                        label: step.label,
                        focal: step.focal,
                    });
                }
            }
            _ => unreachable!(),
        }
    }

    let mut total: u64 = 0;
    for (b, v) in map.iter().enumerate() {
        for (s, l) in v.iter().enumerate() {
            total += (b as u64 + 1) * (s as u64 + 1) * l.focal;
        }
    }
    Some(total)
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = read_to_string(filename).unwrap().replace('\n', "");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::{holliday_hash, part1, part2};

    #[test]
    fn test_hash() {
        let input = "HASH";
        assert_eq!(holliday_hash(&input), 52);
    }

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), Some(145));
    }
}
