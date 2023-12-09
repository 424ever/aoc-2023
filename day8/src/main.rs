use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};

use num::Integer;

struct Node {
    name: String,
    next: (String, String),
}

fn parse_nodes(lines: Vec<String>) -> HashMap<String, Node> {
    let mut map = HashMap::new();

    for line in lines {
        let line = line.to_string();
        let split = line.split_once('=').unwrap();
        let from = split.0.trim()[0..3].to_string();
        let to = &split.1.replace("(", "").replace(")", "").replace(" ", "");
        let to = to.split_once(',').unwrap();
        let to = (to.0[0..3].to_string(), to.1[0..3].to_string());

        map.insert(
            from.clone(),
            Node {
                name: from,
                next: to,
            },
        );
    }

    map
}

fn count_from_start(
    directions: &Vec<char>,
    nodes: &HashMap<String, Node>,
    start: &str,
    end_on_z_at_end: bool,
) -> Option<u32> {
    let mut node = nodes.get(start)?;
    let mut count = 0;
    let mut directions = directions.iter().cycle();

    loop {
        let dir = directions.next();
        match dir? {
            'L' => node = nodes.get(&node.next.0)?,
            'R' => node = nodes.get(&node.next.1)?,
            _ => unreachable!(),
        }

        count += 1;

        if end_on_z_at_end && node.name.ends_with('Z') {
            break;
        } else if node.name == "ZZZ" {
            break;
        }
    }

    Some(count)
}

fn part_1(directions: &Vec<char>, nodes: &HashMap<String, Node>) -> Option<u32> {
    count_from_start(directions, nodes, "AAA", false)
}

fn part_2(directions: &Vec<char>, nodes: &HashMap<String, Node>) -> Option<u64> {
    Some(
        nodes
            .iter()
            .filter(|(n, _)| n.ends_with('A'))
            .map(|(n, _)| count_from_start(directions, nodes, n, true).unwrap() as u64)
            .fold(1u64, |acc, elem| acc.lcm(&elem)),
    )
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let directions: Vec<char> = lines.nth(0).unwrap().chars().collect();
    let nodes = parse_nodes(lines.skip(1).collect());

    println!("{}", part_1(&directions, &nodes).unwrap());
    println!("{}", part_2(&directions, &nodes).unwrap());
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{parse_nodes, part_1, part_2};

    #[test]
    fn test_part_1_1() {
        let mut lines: Vec<String> = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .iter()
        .map(|l| String::from(*l))
        .collect();
        let directions: Vec<char> = lines.get(0).unwrap().chars().collect();
        lines.remove(0);
        lines.remove(0);
        let nodes = parse_nodes(lines);

        assert_eq!(part_1(&directions, &nodes), Some(2));
    }

    #[test]
    fn test_part_1_2() {
        let mut lines: Vec<String> = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .iter()
        .map(|l| String::from(*l))
        .collect();
        let directions: Vec<char> = lines.get(0).unwrap().chars().collect();
        lines.remove(0);
        lines.remove(0);
        let nodes = parse_nodes(lines);

        assert_eq!(part_1(&directions, &nodes), Some(6));
    }

    #[test]
    fn test_part_2() {
        let mut lines: Vec<String> = vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ]
        .iter()
        .map(|l| String::from(*l))
        .collect();
        let directions: Vec<char> = lines.get(0).unwrap().chars().collect();
        lines.remove(0);
        lines.remove(0);
        let nodes = parse_nodes(lines);

        assert_eq!(part_2(&directions, &nodes), Some(6));
    }
}
