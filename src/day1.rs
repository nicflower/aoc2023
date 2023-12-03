use std::path::Path;

pub async fn part1(input_file: &Path) -> anyhow::Result<u64> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let sum = content.lines().fold(0, |acc, line| {
        let calib_number = line_to_calib_num(line).unwrap_or(0);
        acc + calib_number
    });
    Ok(sum)
}
pub async fn part2(input_file: &Path) -> anyhow::Result<u64> {
    let content = tokio::fs::read_to_string(input_file).await?;
    let sum = content.lines().map(|line| map_spelled_digits(line)).filter(|res| res.is_ok()).map(|el| el.unwrap()).fold(0, |acc, line| {
        let calib_number = line_to_calib_num(&line).unwrap_or(0);
        acc + calib_number
    });
    Ok(sum)
}

fn line_to_calib_num(line: &str) -> anyhow::Result<u64> {
    let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
    let num = match digits.len() {
        0 => 0,
        1 => format!("{}{}", digits.first().unwrap(), digits.first().unwrap()).parse::<u64>()?,
        _ => format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<u64>()?,
    };
    Ok(num)
}

const DIGITS_SPELL: &[(&str, char)] = &[
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn map_spelled_digits(line: &str) -> anyhow::Result<String> {
    let mut out_indx = 0;
    // find spelled digits
    let mut ret_str = String::with_capacity(line.len());
    while out_indx < line.chars().count() {
        let digit = {
            let mut found_digit = None;
            for digit in DIGITS_SPELL {
                if out_indx + digit.0.len() <= line.len()
                    && &line[out_indx..out_indx + digit.0.len()] == digit.0
                {
                    found_digit = Some(digit.1);
                    break;
                }
            }
            found_digit
        };
        match digit {
            Some(d) => {
                ret_str.push(*d);
            }
            None => {
                ret_str.push(line.chars().nth(out_indx).unwrap());
            }
        };
        out_indx += 1;
    }
    Ok(ret_str)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn calib_no_digit() {
        let calib_num_res = line_to_calib_num("asdfg");
        assert!(calib_num_res.is_ok());
        assert_eq!(calib_num_res.unwrap(), 0);
    }
    #[test]
    fn calib_single_digit() {
        let calib_num_res = line_to_calib_num("as1bnb");
        assert!(calib_num_res.is_ok());
        assert_eq!(calib_num_res.unwrap(), 11);
    }

    #[test]
    fn calib_multi_digit() {
        let calib_num_res = line_to_calib_num("as76123nb");
        assert!(calib_num_res.is_ok());
        assert_eq!(calib_num_res.unwrap(), 73);
    }
    #[test]
    fn map_spelled_none() {
        let map_spelled = map_spelled_digits("aszeven23");
        assert!(map_spelled.is_ok());
        assert_eq!(map_spelled.unwrap(), "aszeven23");
    }
    #[test]
    fn map_spelled_single() {
        let map_spelled = map_spelled_digits("asseven23");
        assert!(map_spelled.is_ok());
        assert_eq!(map_spelled.unwrap(), "as723");
    }
    #[test]
    fn map_spelled_multi() {
        let map_spelled = map_spelled_digits("asseven23onetwoeight");
        assert!(map_spelled.is_ok());
        assert_eq!(map_spelled.unwrap(), "as723128");
    }
}
