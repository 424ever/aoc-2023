use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};

struct Game {
    time: u64,
    record: u64,
}

impl Game {
    fn count_above_record(&self) -> u64 {
        (1..self.time)
            .map(|i| i * (self.time - i))
            .filter(|d| d > &self.record)
            .count() as u64
    }
}

fn parse_line(line: &str) -> Option<Vec<u64>> {
    line.split_once(':')?
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .map(|s| s.parse())
        .map(|r| r.ok())
        .collect()
}

fn parse_games(lines: &Vec<String>) -> Option<Vec<Game>> {
    let times = lines.get(0)?;
    let distances = lines.get(1)?;

    let times = parse_line(times)?;
    let distances = parse_line(distances)?;

    let games: Vec<Game> = times
        .iter()
        .zip(distances)
        .map(|(time, record)| Game {
            time: *time,
            record,
        })
        .collect();
    Some(games)
}

fn parse_game_no_space(lines: &Vec<String>) -> Option<Game> {
    Some(Game {
        time: lines
            .get(0)?
            .split_once(':')?
            .1
            .replace(" ", "")
            .parse()
            .ok()?,
        record: lines
            .get(1)?
            .split_once(':')?
            .1
            .replace(" ", "")
            .parse()
            .ok()?,
    })
}

fn part1(games: &Vec<Game>) -> u64 {
    games.iter().map(|g| g.count_above_record()).product()
}

fn part2(game: &Game) -> u64 {
    game.count_above_record()
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(ErrorKind::Other)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let part1 = part1(&parse_games(&lines).ok_or(ErrorKind::Other)?);
    let part2 = part2(&parse_game_no_space(&lines).ok_or(ErrorKind::Other)?);
    println!("{}\n{}", part1, part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_game_no_space, parse_games, part1, part2};

    #[test]
    fn test_part_1() {
        let lines = ["Time:      7  15   30", "Distance:  9  40  200"];
        let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        let games = parse_games(&lines).unwrap();
        assert_eq!(part1(&games), 288);
    }

    #[test]
    fn test_part_2() {
        let lines = ["Time:      7  15   30", "Distance:  9  40  200"];
        let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        let game = parse_game_no_space(&lines).unwrap();
        assert_eq!(part2(&game), 71503);
    }
}
