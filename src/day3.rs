use std::path::Path;

pub async fn part1(input_file: &Path) -> anyhow::Result<u32> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let lines: Vec<&str> = content.lines().collect();
    let part_symbols = find_symbols(&lines);
    let part_nums = find_part_numbers(&lines, &part_symbols);
    let sum = part_nums.iter().map(|el| el.num()).sum::<u32>();
    Ok(sum)
}
pub async fn part2(input_file: &Path) -> anyhow::Result<u32> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let lines: Vec<&str> = content.lines().collect();
    let part_symbols = find_symbols(&lines);
    let part_nums = find_part_numbers(&lines, &part_symbols);

    let gears = lines
        .iter()
        .enumerate()
        .flat_map(|(indx, line)| Gear::from_line(line, &part_nums, indx))
        .collect::<Vec<Gear>>();
    let sum = gears.iter().map(|el| el.ratio()).sum();
    Ok(sum)
}

pub fn find_symbols(lines: &[&str]) -> Vec<char> {
    let mut symbols = vec![];
    for line in lines {
        let mut new_symbols = line
            .chars()
            .filter(|el| !el.is_numeric())
            .filter(|el| *el != '.')
            .filter(|c| !symbols.iter().any(|known_symbol| known_symbol == c))
            .collect();
        symbols.append(&mut new_symbols)
    }

    symbols
}

fn find_part_numbers(lines: &[&str], part_symbols: &[char]) -> Vec<PartNumber> {
    let mut part_numbers = vec![];
    lines.iter().enumerate().for_each(|(indx, line)| {
        let upper_line = if indx == 0 {
            None
        } else {
            Some(lines[indx - 1])
        };
        let lower_line = if indx == (lines.len() - 1) {
            None
        } else {
            Some(lines[indx + 1])
        };
        part_numbers.append(
            &mut PartNumber::read_line(line, upper_line, lower_line, part_symbols, indx)
                .unwrap_or_default(),
        );
    });
    part_numbers
}

#[derive(Debug, Clone)]
struct PartNumber {
    num: u32,
    coordinates: PartNumberCoordinates,
}
impl PartNumber {
    pub fn new(num: u32, coordinates: PartNumberCoordinates) -> Self {
        PartNumber { num, coordinates }
    }

    pub fn num(&self) -> u32 {
        self.num
    }

    fn try_from_line(line: &str, line_indx: usize) -> anyhow::Result<Vec<Self>> {
        let coordinates = PartNumberCoordinates::from_line(line, line_indx);
        coordinates
            .into_iter()
            .map(|el| {
                let num = line[el.start()..=el.end()].parse::<u32>()?;
                Ok(Self::new(num, el))
            })
            .collect()
    }

    pub fn read_line(
        cur_line: &str,
        prev_line: Option<&str>,
        next_line: Option<&str>,
        part_symbols: &[char],
        line_indx: usize,
    ) -> anyhow::Result<Vec<PartNumber>> {
        let part_nums = Self::try_from_line(cur_line, line_indx)?
            .into_iter()
            .filter(|el| {
                // check prev_line
                if let Some(line) = prev_line {
                    if el.coordinates.check_against_line(line, part_symbols) {
                        return true;
                    }
                }
                if let Some(line) = next_line {
                    if el.coordinates.check_against_line(line, part_symbols) {
                        return true;
                    }
                }

                el.coordinates.check_against_line(cur_line, part_symbols)
            })
            .collect::<Vec<PartNumber>>();
        Ok(part_nums)
    }
}
#[derive(Debug, Clone)]
struct PartNumberCoordinates {
    line: usize,
    indx_start: usize,
    indx_end: usize,
}

impl PartNumberCoordinates {
    pub fn new(line: usize, indx_start: usize, indx_end: usize) -> Self {
        Self {
            line,
            indx_start,
            indx_end,
        }
    }

    fn is_symbol_adjacent(&self, line: usize, indx: usize) -> bool {
        if self.line == line {
            let adjacent_before = if self.indx_start > 0 {
                indx == self.indx_start - 1
            } else {
                false
            };
            let adjacent_after = indx == self.indx_end + 1;
            return adjacent_after | adjacent_before;
        } else if (self.line as i16 - line as i16).abs() == 1 {
            let min_indx = if self.indx_start > 0 {
                self.indx_start - 1
            } else {
                0
            };
            let max_indx = self.indx_end + 1;
            return indx >= min_indx && indx <= max_indx;
        }
        false
    }

    fn from_line(line: &str, line_indx: usize) -> Vec<PartNumberCoordinates> {
        let nums: Vec<(usize, char)> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_numeric())
            .collect();

        if nums.is_empty() {
            return vec![];
        }

        let (mut beg_indx, mut end_indx) = (nums[0].0, nums[0].0);
        let mut coordinates = Vec::with_capacity(nums.len() / 2);
        for (indx, _) in nums.iter() {
            if indx - end_indx > 1 {
                coordinates.push(PartNumberCoordinates::new(line_indx, beg_indx, end_indx));
                beg_indx = *indx;
            }
            end_indx = *indx;
        }
        coordinates.push(PartNumberCoordinates::new(line_indx, beg_indx, end_indx));
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

struct Gear {
    part_nums: [PartNumber; 2],
}

impl Gear {
    pub fn new(part_nums: [PartNumber; 2]) -> Self {
        Self { part_nums }
    }

    pub fn ratio(&self) -> u32 {
        self.part_nums[0].num * self.part_nums[1].num
    }
    pub fn from_line(line: &str, part_numbers: &[PartNumber], line_indx: usize) -> Vec<Gear> {
        let candidate_gears_pos: Vec<usize> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '*')
            .map(|(indx, _)| indx)
            .collect();

        candidate_gears_pos
            .iter()
            .filter_map(|indx| {
                let part_nums: Vec<&PartNumber> = part_numbers
                    .iter()
                    .filter(|el| el.coordinates.is_symbol_adjacent(line_indx, *indx))
                    .collect();
                if part_nums.len() != 2 {
                    return None;
                }
                Some(Self::new([part_nums[0].clone(), part_nums[1].clone()]))
            })
            .collect()
    }
}
#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn coordinates_reader() {
        let str = "467..114..";
        let coordinates = PartNumberCoordinates::from_line(str, 0);
        assert_eq!(coordinates[0].indx_start, 0);
        assert_eq!(coordinates[0].indx_end, 2);
        assert_eq!(coordinates.len(), 2);
    }
    fn find_part_numbers_on_line() {
        let line1 = "...*......";
        let line2 = "..35..633.";
        let line3 = "......#...";
        let read_res = PartNumber::read_line(line2, Some(line1), Some(line3), &['*', '#'], 1);
        assert!(read_res.is_ok(), "failed to read part numbers");
        let nums = read_res.unwrap();
        assert_eq!(nums.len(), 2);
        assert_eq!(nums[0].num(), 35);
        assert_eq!(nums[1].num(), 633);
    }
}
