mod stuff;

use solarized::{
    clear, print_colored,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    PrintMode::NewLine
};
use stuff::{read_config, print_log_contents};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear();
    print_colored(
        &["Welcome ", "to ", "the ", "Eve ", "log ", "parser ", "of ", "doom."],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA],
        NewLine,
    );
    let (log_dir_path, search_words, stats) = read_config()?;
    let most_recent_file_path = stuff::find_most_recent_log_file(&log_dir_path)?;
    println!("Reading from file: {:?}", most_recent_file_path);
    let path = Arc::new(most_recent_file_path);
    let search_words = Arc::new(search_words);
    let stats = Arc::new(stats);
    print_log_contents(&path, &search_words, &stats);
    Ok(())
}
