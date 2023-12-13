use std::path::Path;

pub async fn part1(input_file: &Path) -> anyhow::Result<u32> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let lines: Vec<&str> = content.lines().map(|el| el).collect();
    let part_symbols = find_symbols(&lines);
    let part_nums = find_part_numbers(&lines, &part_symbols);
    let sum = part_nums.iter().map(|el| el.num()).sum::<u32>();
    Ok(sum)
}

pub fn find_symbols(lines: &[&str]) -> Vec<char> {
    let mut symbols = vec![];
    for line in lines {
        let mut new_symbols = line.chars().filter(|el| !el.is_numeric()).filter(|el| *el != '.').filter(|c| {
            !symbols.iter().any(|known_symbol| known_symbol == c)
        }).collect();
        symbols.append(&mut new_symbols)
    }

    symbols
}

fn find_part_numbers(lines: &[&str], part_symbols: &[char]) -> Vec<PartNumber> {
    let mut part_numbers = vec![];
    lines.iter().enumerate().for_each(|(indx, line)| {
        let upper_line = if indx == 0 {None} else {Some(lines[indx-1])};
        let lower_line= if indx == (lines.len() - 1) {None} else {Some(lines[indx + 1])};
        part_numbers.append(&mut PartNumber::read_line(line, upper_line, lower_line, part_symbols).unwrap_or_default());
    });
    part_numbers
}

#[derive(Debug)]
struct PartNumber {
    num: u32,
}
#[derive(Debug)]
struct PartNumberCoordinates {
    indx_start: usize,
    indx_end: usize,
}

impl PartNumberCoordinates {
    pub fn new(indx_start: usize, indx_end: usize) -> Self {
        Self {
            indx_start,
            indx_end,
        }
    }

    fn from_line(line: &str) -> Vec<PartNumberCoordinates> {
        let nums: Vec<(usize, char)> = line.chars().enumerate().filter(|(_, c)| {c.is_numeric()}).collect();

        if nums.is_empty() {
            return vec![];
        }

        let (mut beg_indx, mut end_indx) = (nums[0].0, nums[0].0);
        let mut coordinates = Vec::with_capacity(nums.len() / 2);
        for (indx, _) in nums.iter() {
            if indx - end_indx > 1 {
                coordinates.push(PartNumberCoordinates::new(beg_indx, end_indx));
                beg_indx = *indx;
            }
            end_indx = *indx;
        }
        coordinates.push(PartNumberCoordinates::new(beg_indx, end_indx));
        coordinates
    }

    pub fn check_against_line(&self, line: &str, part_symbols: &[char]) -> bool {
        let start = if self.indx_start > 0 {
            self.indx_start - 1
        } else {
            0
        };
        let end = if self.indx_end < line.len() - 1 {
            self.indx_end + 1
        } else {
            line.len() - 1
        };
        line[start..=end]
            .chars()
            .filter(|c| part_symbols.iter().any(|el| el == c))
            .count()
            != 0
    }

    pub fn start(&self) -> usize {
        self.indx_start
    }

    pub fn end(&self) -> usize {
        self.indx_end
    }
}

impl PartNumber {
    pub fn new(num: u32) -> Self {
        PartNumber { num }
    }

    pub fn num(&self) -> u32 {
        self.num
    }

    fn try_from_coordinates_and_line(
        coordinates: &PartNumberCoordinates,
        line: &str,
    ) -> anyhow::Result<Self> {
        let num = line[coordinates.start()..=coordinates.end()].parse::<u32>()?;
        Ok(Self::new(num))
    }

    pub fn read_line(
        cur_line: &str,
        prev_line: Option<&str>,
        next_line: Option<&str>,
        part_symbols: &[char]
    ) -> anyhow::Result<Vec<PartNumber>> {
        let coordinates = PartNumberCoordinates::from_line(cur_line);
        let part_nums = coordinates
            .iter()
            .filter(|el| {
                // check prev_line
                if let Some(line) = prev_line {
                    if el.check_against_line(line, part_symbols) {
                        return true;
                    }
                }
                if let Some(line) = next_line {
                    if el.check_against_line(line, part_symbols) {
                        return true;
                    }
                }

                el.check_against_line(cur_line, part_symbols)
            })
            .map(|el| Self::try_from_coordinates_and_line(el, cur_line))
            .collect::<Result<Vec<PartNumber>, _>>().unwrap();
        Ok(part_nums)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn coordinates_reader() {
        let str = "467..114..";
        let coordinates = PartNumberCoordinates::from_line(str);
        assert_eq!(coordinates[0].indx_start, 0);
        assert_eq!(coordinates[0].indx_end, 2);
        assert_eq!(coordinates.len(), 2);
    }
    #[test]
    fn find_num_generic() {
        let str = "467..114..";
        let read_res = PartNumber::read_line(str, None, None);
        assert!(read_res.is_ok(), "failed to read part numbers");
        let nums = read_res.unwrap();
        //assert_eq!(nums[0].num(), 467);
        assert_eq!(nums[1].num(), 114);
    }

    fn find_part_numbers_on_line() {
        let line1 = "...*......";
        let line2 = "..35..633.";
        let line3 = "......#...";
        let read_res = PartNumber::read_line(line2, Some(line1), Some(line3));
        assert!(read_res.is_ok(), "failed to read part numbers");
        let nums = read_res.unwrap();
        assert_eq!(nums.len(), 2);
        assert_eq!(nums[0].num(), 35);
        assert_eq!(nums[1].num(), 633);
    }
}
