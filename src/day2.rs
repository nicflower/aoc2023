use anyhow::anyhow;
use std::path::Path;

pub async fn part1(input_file: &Path) -> anyhow::Result<u32> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let (max_r, max_g, max_b) = (12, 13, 14);
    let id_sum = content
        .lines()
        .map(GameOutcome::try_from)
        .filter_map(|el| el.ok())
        .filter(|el| el.is_possibile(max_r, max_g, max_b))
        .map(|el| el.id)
        .sum::<u32>();
    Ok(id_sum)
}
pub async fn part2(input_file: &Path) -> anyhow::Result<u32> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let power_sum = content
        .lines()
        .map(GameOutcome::try_from)
        .filter_map(|el| el.ok())
        .map(|el| el.min_disposition().power())
        .sum::<u32>();
    Ok(power_sum)
}

#[derive(Debug, Default, PartialEq)]
struct CubesDisposition {
    red: Option<u8>,
    green: Option<u8>,
    blue: Option<u8>,
}

impl CubesDisposition {
    pub fn power(&self) -> u32 {
        self.red.unwrap_or(1) as u32
            * self.green.unwrap_or(1) as u32
            * self.blue.unwrap_or(1) as u32
    }
}

impl CubesDisposition {
    pub fn new(red: Option<u8>, green: Option<u8>, blue: Option<u8>) -> Self {
        Self { red, green, blue }
    }
}

const COLOR_STR: (&str, &str, &str) = ("red", "green", "blue");

impl TryFrom<&str> for CubesDisposition {
    type Error = anyhow::Error;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        enum CubeColor {
            Red(u8),
            Green(u8),
            Blue(u8),
        }
        let (red, green, blue) = str
            .split(',')
            .map(|substr| {
                let num = substr
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u8>()?;
                if substr.contains(COLOR_STR.0) {
                    Ok(CubeColor::Red(num))
                } else if substr.contains(COLOR_STR.1) {
                    Ok(CubeColor::Green(num))
                } else if substr.contains(COLOR_STR.2) {
                    Ok(CubeColor::Blue(num))
                } else {
                    Err(anyhow!("Could not find a matching color in {}", substr))
                }
            })
            .collect::<Result<Vec<CubeColor>, _>>()?
            .iter()
            .fold((0, 0, 0), |acc, el| match el {
                CubeColor::Red(n) => (acc.0 + n, acc.1, acc.2),
                CubeColor::Green(n) => (acc.0, acc.1 + n, acc.2),
                CubeColor::Blue(n) => (acc.0, acc.1, acc.2 + n),
            });
        Ok(Self::new(
            if red > 0 { Some(red) } else { None },
            if green > 0 { Some(green) } else { None },
            if blue > 0 { Some(blue) } else { None },
        ))
    }
}

#[derive(Debug, Default, PartialEq)]
struct GameOutcome {
    id: u32,
    dispositions: Vec<CubesDisposition>,
}

impl GameOutcome {
    pub fn is_possibile(&self, max_r: u8, max_g: u8, max_b: u8) -> bool {
        self.dispositions
            .iter()
            .map(|el| {
                el.red.unwrap_or_default() <= max_r
                    && el.green.unwrap_or_default() <= max_g
                    && el.blue.unwrap_or_default() <= max_b
            })
            .filter(|el| !*el)
            .count()
            == 0
    }

    pub fn min_disposition(&self) -> CubesDisposition {
        let min_r = self
            .dispositions
            .iter()
            .map(|el| el.red)
            .max()
            .unwrap_or_default();
        let min_g = self
            .dispositions
            .iter()
            .map(|el| el.green)
            .max()
            .unwrap_or_default();
        let min_b = self
            .dispositions
            .iter()
            .map(|el| el.blue)
            .max()
            .unwrap_or_default();
        CubesDisposition::new(min_r, min_g, min_b)
    }
}

impl TryFrom<&str> for GameOutcome {
    type Error = anyhow::Error;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let id = line
            .split_once(' ')
            .ok_or(anyhow!("Could not split {} on ' '", line))?
            .1
            .split_once(':')
            .ok_or(anyhow!("Could not split {} on ':' ", line))?
            .0
            .parse::<u32>()?;
        let cubes_disposition_str = line
            .split_once(':')
            .ok_or(anyhow!("Could not split {} on ':' ", line))?
            .1;

        Ok(Self {
            id,
            dispositions: cubes_disposition_str
                .split(';')
                .map(CubesDisposition::try_from)
                .collect::<Result<Vec<CubesDisposition>, _>>()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_game_single_disposition() {
        let line = "Game 1: 3 blue, 4 red";
        let parse_res: Result<GameOutcome, _> = line.try_into();
        assert!(
            parse_res.is_ok(),
            "{}",
            format!("parsing error: {}", parse_res.unwrap_err())
        );
        let game = parse_res.unwrap();
        let expected_game = GameOutcome {
            id: 1,
            dispositions: vec![CubesDisposition::new(Some(4), None, Some(3))],
        };
        assert_eq!(game, expected_game);
    }
    #[test]
    fn parse_game_multi_disposition() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let parse_res: Result<GameOutcome, _> = line.try_into();
        assert!(
            parse_res.is_ok(),
            "{}",
            format!("parsing error: {}", parse_res.unwrap_err())
        );
        let game = parse_res.unwrap();
        let expected_game = GameOutcome {
            id: 3,
            dispositions: vec![
                CubesDisposition::new(Some(20), Some(8), Some(6)),
                CubesDisposition::new(Some(4), Some(13), Some(5)),
                CubesDisposition::new(Some(1), Some(5), None),
            ],
        };
        assert_eq!(game, expected_game);
    }
    #[test]
    fn parse_disposition() {
        let line = " 1 red, 2 green, 6 blue";
        let parse_res: Result<CubesDisposition, _> = line.try_into();
        assert!(
            parse_res.is_ok(),
            "{}",
            format!("parsing error: {}", parse_res.unwrap_err())
        );
        let cubes = parse_res.unwrap();
        assert_eq!(cubes.red, Some(1), "checking red cubes");
        assert_eq!(cubes.green, Some(2), "checking green cubes");
        assert_eq!(cubes.blue, Some(6), "checking blue cubes");
    }
}
