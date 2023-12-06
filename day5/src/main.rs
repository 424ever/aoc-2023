mod map;

use std::{
    env,
    fs::File,
    io::BufReader,
    io::{BufRead, ErrorKind},
    ops::Range,
};

use itertools::Itertools;
use map::range_from_start_len;
use map::Map;

fn seeds_from_line(line: &String) -> Option<Vec<u64>> {
    line.split_once(':')?
        .1
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>())
        .map(|r| r.ok())
        .collect()
}

fn seed_ranges_from_line(line: &String) -> Option<Vec<Range<u64>>> {
    let mut seeds: Vec<Range<u64>> = vec![];
    for chunk in &seeds_from_line(line)?.iter().chunks(2) {
        let range = chunk.collect::<Vec<&u64>>();
        let range: Vec<u64> = range.iter().map(|x| **x).collect();
        let min = range[0];
        let len = range[1];
        seeds.push(range_from_start_len(min, len));
    }
    return Some(seeds);
}

fn maps_from_iter<'a, I>(iter: I) -> Option<Vec<Map>>
where
    I: Iterator<Item = &'a String>,
{
    iter.group_by(|elt| elt.is_empty())
        .into_iter()
        .filter_map(|(key, group)| if !key { Some(group) } else { None })
        .map(|g| g.map(|r| r.to_owned()).collect::<Vec<String>>())
        .map(Map::try_from)
        .map(|r| r.ok())
        .collect()
}

fn find_min_after_apply_single(seeds: &Vec<u64>, maps: &Vec<Map>) -> Option<u64> {
    Some(
        seeds
            .iter()
            .map(|s| -> u64 {
                let mut i: u64 = *s;
                for map in maps.iter() {
                    i = map.apply(i);
                }
                i
            })
            .min()?,
    )
}

fn find_min_after_apply_ranges(seeds: &Vec<Range<u64>>, maps: &Vec<Map>) -> Option<u64> {
    let mut ranges = seeds.clone();
    for map in maps {
        map.apply_ranges(&mut ranges);
    }
    ranges.iter().map(|r| r.start).min()
}

fn part1(lines: &Vec<String>) -> Option<u64> {
    let seeds = seeds_from_line(lines.get(0)?)?;
    let maps = maps_from_iter(lines.iter().skip(2))?;
    find_min_after_apply_single(&seeds, &maps)
}

fn part2(lines: &Vec<String>) -> Option<u64> {
    let ranges = seed_ranges_from_line(lines.get(0)?)?;
    let maps = maps_from_iter(lines.iter().skip(2))?;
    find_min_after_apply_ranges(&ranges, &maps)
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let part1 = part1(&lines);
    let part2 = part2(&lines);
    println!(
        "{}\n{}",
        part1.ok_or(ErrorKind::Other)?,
        part2.ok_or(ErrorKind::Other)?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        let lines = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
        let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        assert_eq!(part1(&lines).unwrap(), 35);
    }

    #[test]
    fn test_part_2() {
        let lines = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
        let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        assert_eq!(part2(&lines).unwrap(), 46)
    }
}
