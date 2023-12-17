use anyhow::anyhow;
use std::path::Path;
pub mod day1;
pub mod day2;
pub mod day3;
mod day4;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("A day must be provided via cli"));
    }
    let day = args[1].parse::<u8>()?;
    match day {
        1 => {
            let input_path = Path::new("./input/day1.txt");
            let p1_answer = day1::part1(input_path).await?;
            println!("Day 1 part 1: {p1_answer}");
            let p2_answer = day1::part2(input_path).await?;
            println!("Day 1 part 2: {p2_answer}");
        }
        2 => {
            let input_path = Path::new("./input/day2.txt");
            let p1_answer = day2::part1(input_path).await?;
            println!("Day 2 part 1: {p1_answer}");
            let p2_answer = day2::part2(input_path).await?;
            println!("Day 2 part 2: {p2_answer}");
        }
        3 => {
            let input_path = Path::new("./input/day3.txt");
            let p1_answer = day3::part1(input_path).await?;
            println!("Day 3 part 1: {p1_answer}");
            let p2_answer = day3::part2(input_path).await?;
            println!("Day 3 part 2: {p2_answer}");
        }
        4 => {
            let input_path = Path::new("./input/day4.txt");
            let p1_answer = day4::part1(input_path).await?;
            println!("Day 4 part 1: {p1_answer}");
            let p2_answer = day4::part2(input_path).await?;
            println!("Day 4 part 2: {p2_answer}");
        }
        _ => return Err(anyhow!("{} is not a valid day value", day)),
    };
    Ok(())
}
