use days::days_module::day::Day;
use days::days_module::day_01::Day01;
use days::days_module::day_02::Day02;
use days::days_module::day_03::Day03;
use days::days_module::day_04::Day04;
use days::days_module::day_05::Day05;
use days::days_module::day_06::Day06;
use days::days_module::day_07::Day07;
use days::days_module::day_08::Day08;
use days::days_module::day_09::Day09;
use days::days_module::day_10::Day10;
use days::days_module::day_11::Day11;
use days::days_module::day_12::Day12;
use days::days_module::day_13::Day13;
use days::days_module::day_14::Day14;
use days::days_module::day_15::Day15;
use days::days_module::day_16::Day16;
use days::days_module::day_17::Day17;
use days::days_module::day_18::Day18;
use days::days_module::day_19::Day19;
use days::days_module::day_20::Day20;
use days::days_module::day_21::Day21;
use days::days_module::day_22::Day22;
use days::days_module::day_23::Day23;
use days::days_module::day_24::Day24;
use days::days_module::day_25::Day25;
use helpers::fetch_input_with_cache;
use helpers::submit_answer;
use std::time::Instant;
use tokio;

async fn execute_day(day: &Box<dyn Day>) -> Result<(String, String), Box<dyn std::error::Error>> {
    let input = fetch_input_with_cache(2024, day.get_index()).await?;
    let day_index = day.get_index();
    let solution_a = day.part_a(&input);
    submit_answer(2024, day_index, 1, &solution_a).await?;
    let solution_b = day.part_b(&input);
    submit_answer(2024, day_index, 2, &solution_b).await?;
    Ok((solution_a, solution_b))
}

#[tokio::main]
async fn main() {
    let mut days: Vec<Box<dyn Day>> = Vec::new();
    days.push(Box::new(Day01 {}));
    days.push(Box::new(Day02 {}));
    days.push(Box::new(Day03 {}));
    days.push(Box::new(Day04 {}));
    days.push(Box::new(Day05 {}));
    days.push(Box::new(Day06 {}));
    days.push(Box::new(Day07 {}));
    days.push(Box::new(Day08 {}));
    days.push(Box::new(Day09 {}));
    days.push(Box::new(Day10 {}));
    days.push(Box::new(Day11 {}));
    days.push(Box::new(Day12 {}));
    days.push(Box::new(Day13 {}));
    days.push(Box::new(Day14 {}));
    days.push(Box::new(Day15 {}));
    days.push(Box::new(Day16 {}));
    days.push(Box::new(Day17 {}));
    days.push(Box::new(Day18 {}));
    days.push(Box::new(Day19 {}));
    days.push(Box::new(Day20 {}));
    days.push(Box::new(Day21 {}));
    days.push(Box::new(Day22 {}));
    days.push(Box::new(Day23 {}));
    days.push(Box::new(Day24 {}));
    days.push(Box::new(Day25 {}));

    let start = Instant::now();
    println!(
        "{0: <4}   | {1: <20} | {2: <20} | {3: <20}",
        "Day", "Part A", "Part B", "Runtime"
    );
    for day in days {
        // Start the timer!
        let now = Instant::now();

        // Run the parts.
        let solutions = execute_day(&day).await.unwrap();

        let runtime = format!(
            "{}.{} ms",
            now.elapsed().as_millis(),
            now.elapsed().as_nanos() % 1000000
        );
        println!(
            "{0: <4} | {1: <20} | {2: <20} | {3: <20}",
            day.get_id(),
            solutions.0,
            solutions.1,
            runtime,
        );
    }
    println!("\nTotal {} ms", start.elapsed().as_millis());
}
