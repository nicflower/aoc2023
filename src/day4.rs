use anyhow::anyhow;
use std::path::Path;
pub async fn part1(input_file: &Path) -> anyhow::Result<u32> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let lines: Vec<&str> = content.lines().collect();
    let scartch_cards = lines
        .iter()
        .map(|line| ScratchCard::try_from_line(line))
        .collect::<Result<Vec<ScratchCard>, _>>()?;
    let total_score = scartch_cards.iter().map(|card| card.score()).sum();
    Ok(total_score)
}
#[derive(Debug)]
struct ScratchCard {
    id: u16,
    winning_numbers: Vec<u8>,
    my_numbers: Vec<u8>,
}

impl ScratchCard {
    pub fn new(winning_numbers: Vec<u8>, my_numbers: Vec<u8>, id: u16) -> Self {
        Self {
            winning_numbers,
            my_numbers,
            id,
        }
    }

    pub fn score(&self) -> u32 {
        let matching_nums = self
            .winning_numbers
            .iter()
            .filter(|el| self.my_numbers.iter().any(|my_num| my_num == *el))
            .count();

        if matching_nums == 0 {
            return 0;
        }

        2_u32.pow(matching_nums as u32 - 1)
    }

    fn try_from_line(line: &str) -> anyhow::Result<Self> {
        let (front, back) = line
            .split_once(':')
            .ok_or(anyhow!("Could not split {} at ':'", line))?;
        let id = front
            .chars()
            .filter(|el| el.is_numeric())
            .collect::<String>()
            .parse::<u16>()?;
        let (win_nr_str, my_nr_str) = back
            .split_once('|')
            .ok_or(anyhow!("Could not split {} at '|'", line))?;

        let winning_numbers = win_nr_str
            .split_whitespace()
            .map(|el| el.parse::<u8>())
            .collect::<Result<Vec<u8>, _>>()?;
        let my_numbers = my_nr_str
            .split_whitespace()
            .map(|el| el.parse::<u8>())
            .collect::<Result<Vec<u8>, _>>()?;
        Ok(Self::new(winning_numbers, my_numbers, id))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn parse_line() {
        let res = ScratchCard::try_from_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert!(
            res.is_ok(),
            "Failed to parse ScratchCard from line: {}",
            res.unwrap_err()
        );
        let scratch_card = res.unwrap();
        assert_eq!(scratch_card.id, 1, "Scratch card id");
        assert_eq!(
            scratch_card.winning_numbers,
            [41, 48, 83, 86, 17],
            "Winning numbers"
        );
        assert_eq!(
            scratch_card.my_numbers,
            [83, 86, 6, 31, 17, 9, 48, 53],
            "My numbers"
        );
        assert_eq!(scratch_card.score(), 8, "Score");
    }
}
