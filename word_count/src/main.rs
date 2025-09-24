use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut word_count = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            *word_count.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    println!("word count");

    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }

    Ok(())
}

