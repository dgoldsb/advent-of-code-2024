pub mod cube;
pub mod cycle;
pub mod grid;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct SubmissionRecord {
    submissions: HashMap<String, String>,
}

impl SubmissionRecord {
    // Load from file or create a new one if it doesn't exist
    fn load(cache_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = format!("{}/answers.json", cache_dir);
        if Path::new(&file_path).exists() {
            let content = fs::read_to_string(file_path)?;
            let record: SubmissionRecord = serde_json::from_str(&content)?;
            Ok(record)
        } else {
            Ok(Self {
                submissions: HashMap::new(),
            })
        }
    }

    // Save the record to disk
    fn save(&self, cache_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/answers.json", cache_dir);
        let content = serde_json::to_string_pretty(self)?;
        fs::write(file_path, content)?;
        Ok(())
    }

    // Record a new submission
    fn record_submission(&mut self, day: u8, level: u8, answer: &str) {
        let key = format!("day{}_level{}", day, level);
        self.submissions.insert(key, answer.to_string());
    }
}
pub async fn fetch_input_with_cache(
    year: u16,
    day: u8,
) -> Result<String, Box<dyn std::error::Error>> {
    let session_cookie = env::var("AOC_SESSION")?;

    // Define the cache file path
    let cache_dir = "./.cache";
    let cache_file = format!("{}/{}_day{}.txt", cache_dir, year, day);

    // Check if the cache file exists
    if Path::new(&cache_file).exists() {
        // Read from cache
        let cached_input = fs::read_to_string(&cache_file)?;
        return Ok(cached_input);
    }

    // If not cached, fetch the input
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_string();

    // Save the input to the cache
    fs::create_dir_all(cache_dir)?;
    let mut file = fs::File::create(&cache_file)?;
    file.write_all(response.as_bytes())?;

    Ok(response)
}

pub async fn submit_answer(
    year: u16,
    day: u8,
    level: u8,
    answer: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    if answer == "" {
        return Ok("No answer given".to_string());
    }

    let session_cookie = env::var("AOC_SESSION")?;
    let cache_dir = "./.cache";

    // Load submission record
    let mut record = SubmissionRecord::load(cache_dir)?;

    // Check if an answer was already submitted
    let key = format!("day{}_level{}", day, level);
    if let Some(cached_answer) = record.submissions.get(&key) {
        if cached_answer == answer {
            return Ok("Already submitted.".to_string());
        } else {
            println!(
                "Different answer detected for day {}, level {}. \
                Previously submitted: '{}', current answer: '{}'. Skipping submission.",
                day, level, cached_answer, answer
            );
            return Ok("Different answer detected. Skipping submission.".to_string());
        }
    }

    // Submit the answer
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::default()) // Ensure redirects are followed
        .build()?;
    let response = client
        .post(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .header("Content-Type", "application/x-www-form-urlencoded") // Explicit content type
        .form(&[("level", level.to_string()), ("answer", answer.to_string())])
        .send()
        .await?;

    let response_text = response.text().await?;

    if response_text.contains("That's the right answer!") {
        println!("Answer submitted successfully!");
        record.record_submission(day, level, answer);
        record.save(cache_dir)?;
        Ok("Success!".to_string())
    } else if response_text.contains("Oh, hello! Funny seeing you here.") {
        // Treat this as an error and stop submitting further answers
        return Err("You must complete the previous levels first".into());
    } else if response_text.contains("You don't seem to be solving the right level.") {
        return Err("You don't seem to be solving the right level".into());
    } else if response_text.contains("That's not the right answer.") {
        println!("Incorrect answer.");
        return Err("Incorrect answer".into());
    } else if response_text.contains("You gave an answer too recently.") {
        println!("Rate-limited. Try again later.");
        return Err("Rate-limited".into());
    } else {
        println!("Unexpected response: {}", response_text);
        return Err("Unexpected response".into());
    }
}

pub fn read_file(day: String) -> Result<String, String> {
    let file_path = format!("../input/{}.txt", day);

    if let Ok(metadata) = fs::metadata(&file_path) {
        if metadata.is_file() {
            match fs::read_to_string(&file_path) {
                Ok(contents) => Ok(contents),
                Err(err) => Err(format!("Error reading file: {}", err)),
            }
        } else {
            Err("The path does not point to a file.".to_string())
        }
    } else {
        Err("The file does not exist.".to_string())
    }
}

pub fn ints_from_string(input: &str) -> Vec<isize> {
    input
        .split(" ")
        .map(|s| s.parse())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<Vec<isize>>()
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn replace_nth_char_ascii(s: &mut str, idx: usize, newchar: char) {
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    assert!(idx < s_bytes.len());
    assert!(s_bytes[idx].is_ascii());
    assert!(newchar.is_ascii());
    // we've made sure this is safe.
    s_bytes[idx] = newchar as u8;
}

pub fn transpose_string(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        // If there are no lines, there's nothing to transpose
        return String::new();
    }

    let num_lines = lines.len();
    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut transposed_lines = Vec::with_capacity(max_line_length);

    for i in 0..max_line_length {
        let transposed_line: String = (0..num_lines)
            .map(|j| lines[j].chars().nth(i).unwrap_or(' '))
            .collect();

        transposed_lines.push(transposed_line);
    }

    transposed_lines.join("\n")
}

fn base_lcm(a: u128, b: u128) -> u128 {
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

pub fn lcm(numbers: &Vec<u128>) -> u128 {
    numbers.iter().fold(1, |acc, &x| base_lcm(acc, x))
}

pub fn manhattan_distance(start: &(usize, usize), end: &(usize, usize)) -> usize {
    (max(start.0, end.0) - min(start.0, end.0)) + (max(start.1, end.1) - min(start.1, end.1))
}

pub fn gauss_jordan(mut matrix: Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    if cols != rows + 1 {
        return None; // Invalid matrix dimensions
    }

    for i in 0..rows {
        let mut max_row = i;
        for k in i + 1..rows {
            if matrix[k][i].abs() > matrix[max_row][i].abs() {
                max_row = k;
            }
        }

        if i != max_row {
            matrix.swap(i, max_row);
        }

        if matrix[i][i].abs() < 1e-10 {
            return None; // No unique solution
        }

        let pivot = matrix[i][i];
        for j in 0..cols {
            matrix[i][j] /= pivot;
        }

        for k in 0..rows {
            if k != i {
                let factor = matrix[k][i];
                for j in 0..cols {
                    matrix[k][j] -= factor * matrix[i][j];
                }
            }
        }
    }

    let mut solution = vec![0.0; rows];
    for i in 0..rows {
        solution[i] = matrix[i][cols - 1];
    }

    Some(solution)
}
