use std::fs::{self, File, read_dir, metadata};
use std::io::{self, BufRead, BufReader, SeekFrom, Seek, Write};
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::path::PathBuf;
use toml;
use serde::Deserialize;
use regex::Regex;
use solarized::{
    print_colored, print_fancy,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC,
    PrintMode::NewLine
};

#[derive(Deserialize)]
pub struct Config {
    log_dir_path: PathBuf,
    search_words: Vec<String>,
    stats: Vec<String>,
}

pub fn read_config() -> Result<(PathBuf, Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    print_colored(
        &["f", "i", "l", "e ", "r", "e", "a", "d", ": Config OK"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA],
        NewLine,
    );
    Ok((config.log_dir_path, config.search_words, config.stats))
}

pub fn find_most_recent_log_file(dir_path: &PathBuf) -> io::Result<PathBuf> {
    let mut log_files = Vec::new();
    for entry in read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "txt" {
                let modified_time = metadata(&path)?.modified()?;
                log_files.push((path, modified_time));
            }
        }
    }
    log_files.sort_by(|a, b| b.1.cmp(&a.1));
    log_files.first()
        .map(|(path, _)| path.clone())
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "No log files found"))
}

/*
ESC in ASCII - Start of escape sequence
\x1B
Moves cursor up by 1 line
[ indicates beginning of command
1A instructs terminal to move cursor up one line
[1A
ESC again
\x1B
escape character that clears line from current cursor position to end of line
cursor should be at beginning of line after [1A
[K
print!("\x1B[1A\x1B[K");
println!("Total Bounty: {} ISK", total_bounty);
*/

pub fn print_log_contents(path: Arc<PathBuf>, search_words: Arc<Vec<String>>, stats: Arc<Vec<String>>) {
    let re = Regex::new(r"<.*?>").unwrap();
    let bounty_re = Regex::new(r"\b(\d+),?(\d+)?,?(\d+)? ISK\b").unwrap();
    let mut reader = BufReader::new(File::open(&*path).unwrap());
    let last_pos = 0;
    let mut total_bounty = 0;
    let is_bounty_in_stats = stats.iter().any(|word| word == "bounty" || word == "(bounty)");
    let mut file_output = File::create("output.log").unwrap();
    let mut was_bounty_last_printed = false;
    loop {
        if was_bounty_last_printed {
            print!("\x1B[1A\x1B[K");
            was_bounty_last_printed = false;
        }
        let mut file = File::open(&*path).unwrap();
        file.seek(SeekFrom::Start(last_pos)).unwrap();
        for line in (&mut reader).lines() {
            if let Ok(mut line) = line {
                line = re.replace_all(&line, "").to_string();
                if let Some(captures) = bounty_re.captures(&line) {
                    let mut bounty_str = String::new();
                    for i in 1..=3 {
                        if let Some(m) = captures.get(i) {
                            bounty_str.push_str(m.as_str());
                        }
                    }
                    if let Ok(bounty) = bounty_str.parse::<u64>() {
                        total_bounty += bounty;
                    }
                }
                for search_word in &*search_words {
                    if line.contains(&*search_word) {
                        if let Some((before, after)) = line.split_once(&*search_word) {
                            match &*search_word.as_str() {
                                "(hint)" | "hint" => {
                                    print_fancy(&[
                                        (before, CYAN, vec![]),
                                        (&search_word, YELLOW, vec![BOLD, UNDERLINED, ITALIC]),
                                        (after, CYAN, vec![]),
                                    ], NewLine);
                                }
                                "(combat)" | "combat" => {
                                    print_fancy(&[
                                        (before, CYAN, vec![]),
                                        (&search_word, RED, vec![BOLD, UNDERLINED, ITALIC]),
                                        (after, CYAN, vec![]),
                                    ], NewLine);
                                }
                                "(notify)" | "notify" => {
                                    print_fancy(&[
                                        (before, CYAN, vec![]),
                                        (&search_word, VIOLET, vec![BOLD, UNDERLINED, ITALIC]),
                                        (after, CYAN, vec![]),
                                    ], NewLine);
                                }
                                "(bounty)" | "bounty" => {
                                    print_fancy(&[
                                        (before, CYAN, vec![]),
                                        (&search_word, GREEN, vec![BOLD, UNDERLINED, ITALIC]),
                                        (after, CYAN, vec![]),
                                    ], NewLine);
                                }
                                "(none)" | "none" => {
                                    print_fancy(&[
                                        (before, CYAN, vec![]),
                                        (&search_word, BLUE, vec![BOLD, UNDERLINED, ITALIC]),
                                        (after, CYAN, vec![]),
                                    ], NewLine);
                                }
                                _ => {
                                    let message = format!("{}{}{}", before, &search_word, after);
                                    file_output.write_all(message.as_bytes()).unwrap();
                                    file.write_all(b"\n").unwrap();
                                    print_fancy(&[
                                        (before, CYAN, vec![]),
                                        (&search_word, CYAN, vec![BOLD, UNDERLINED, ITALIC]),
                                        (after, CYAN, vec![]),
                                    ], NewLine);
                                }
                            }
                        }
                    }
                }
            }
        }
        if is_bounty_in_stats {
            if !was_bounty_last_printed {
                let num = format!("{}", total_bounty.to_string());
                let thing = format!("{}", add_commas(&num));
                print_fancy(&[
                    ("Total Bounty: ", CYAN, vec![]),
                    (&thing, GREEN, vec![BOLD]),
                    (" ISK", CYAN, vec![]),
                ], NewLine);
                was_bounty_last_printed = true;
            }
        }
        let _last_pos = file.metadata().map(|m| m.len() as u64).unwrap_or(last_pos);
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_commas(s: &str) -> String {
    let re = Regex::new(r"(\d{3})").unwrap();
    let reversed_string = s.chars().rev().collect::<String>();
    let reversed_with_commas = re.replace_all(&reversed_string, "$1,");
    reversed_with_commas.chars().rev().collect::<String>().trim_start_matches(',').to_string()
}
