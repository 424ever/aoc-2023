use std::cmp::max;
use std::io::BufRead;
use std::{env, fs, str::FromStr};

#[derive(Default)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

struct Config {
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

#[derive(Default)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

#[derive(Debug)]
enum GameParseError {
    InvalidSyntax,
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).ok_or(std::io::ErrorKind::Other)?;
    let file = fs::File::open(filename)?;
    let config_1 = Config {
        max_red: 12,
        max_green: 13,
        max_blue: 14,
    };
    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|s| s.ok()).collect();
    let sum_passed: u32 = lines
        .iter()
        .map(|s| Game::from_str(&s).unwrap())
        .filter(|g| config_1.is_possible(g))
        .fold(0, |a, g| a + g.id);
    let sum_powers: u32 = lines
        .iter()
        .map(|s| Game::from_str(&s).unwrap())
        .map(|g| g.min_power())
        .fold(0, |a, b| a + b);
    println!("{}\n{}", sum_passed, sum_powers);
    Ok(())
}

impl Config {
    fn is_possible(&self, game: &Game) -> bool {
        return game.pulls.iter().all(|p| {
            p.red <= self.max_red && p.green <= self.max_green && p.blue <= self.max_blue
        });
    }
}

impl Game {
    fn min_power(&self) -> u32 {
        let mins = self
            .pulls
            .iter()
            .map(|p| (p.red, p.green, p.blue))
            .reduce(|acc, e| (max(acc.0, e.0), max(acc.1, e.1), max(acc.2, e.2)))
            .unwrap();
        return mins.0 * mins.1 * mins.2;
    }
}

impl FromStr for Game {
    type Err = GameParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Self::default();
        game.id = s
            .split(':')
            .next()
            .ok_or(GameParseError::InvalidSyntax)?
            .split(' ')
            .last()
            .ok_or(GameParseError::InvalidSyntax)?
            .parse()
            .ok()
            .ok_or(GameParseError::InvalidSyntax)?;
        let pulls: Result<Vec<Pull>, _> = s
            .split(':')
            .last()
            .ok_or(GameParseError::InvalidSyntax)?
            .split(';')
            .map(|s| s.trim())
            .map(|s| Pull::from_str(s))
            .collect();
        game.pulls = pulls?;
        return Ok(game);
    }
}

impl FromStr for Pull {
    type Err = GameParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pull = Self::default();
        s.split(',').map(|s| s.trim()).for_each(|s| {
            let num: u32 = s.split(' ').next().unwrap().parse().unwrap();
            let col = s.split(' ').last().unwrap();

            match col {
                "red" => pull.red = num,
                "green" => pull.green = num,
                "blue" => pull.blue = num,
                &_ => {}
            }
        });
        Ok(pull)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Config;
    use crate::Game;

    #[test]
    fn test_part_1() {
        let lines = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let config = Config {
            max_red: 12,
            max_green: 13,
            max_blue: 14,
        };
        let sum_passed: u32 = lines
            .iter()
            .map(|s| Game::from_str(s).unwrap())
            .filter(|g| config.is_possible(g))
            .fold(0, |a, g| a + g.id);
        assert_eq!(sum_passed, 8);
    }

    #[test]
    fn test_part_2() {
        let lines = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let sum_powers: u32 = lines
            .iter()
            .map(|s| Game::from_str(s).unwrap())
            .map(|g| g.min_power())
            .fold(0, |a, b| a + b);
        assert_eq!(sum_powers, 2286);
    }
}
