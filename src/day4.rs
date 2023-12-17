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
pub async fn part2(input_file: &Path) -> anyhow::Result<usize> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let lines: Vec<&str> = content.lines().collect();
    let scartch_cards = lines
        .iter()
        .map(|line| ScratchCard::try_from_line(line))
        .collect::<Result<Vec<ScratchCard>, _>>()?;
    let processed_cards = ScratchCard::bulk_process(scartch_cards);
    Ok(processed_cards.len())
}
#[derive(Debug, Clone)]
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

    pub fn bulk_process(mut cards: Vec<Self>) -> Vec<Self> {
        let mut indx = 0;
        let original_len = cards.len();
        while indx < cards.len() {
            let mut new_cards = cards[indx].process_card(&cards[..original_len]);
            cards.append(&mut new_cards);
            indx += 1;
        }
        cards
    }

    pub fn process_card(&self, all_cards: &[Self]) -> Vec<Self> {
        let matching_nums = self.matching_nums();
        if matching_nums == 0 {
            return vec![];
        }
        let to_find_id = (1..=matching_nums)
            .map(|el| el as u16 + self.id)
            .collect::<Vec<u16>>();
        to_find_id
            .iter()
            .filter_map(|id| all_cards.iter().find(|el| el.id == *id))
            .cloned()
            .collect()
    }

    pub fn matching_nums(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|el| self.my_numbers.iter().any(|my_num| my_num == *el))
            .count()
    }

    pub fn score(&self) -> u32 {
        let matching_nums = self.matching_nums();

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
