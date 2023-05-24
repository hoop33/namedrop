use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_word_file(filename: &str, words: Vec<String>) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for word in words {
        writeln!(file, "{}", word).expect("failed to write word");
    }
    Ok(())
}

fn generate_words() {
    if let Ok(lines) = read_lines("resources/mobypos.txt") {
        let mut nouns = Vec::new();
        let mut adjectives = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split("\\").collect();
                if parts.len() == 2 {
                    if parts[1].contains("N") {
                        nouns.push(parts[0].to_string());
                    }

                    if parts[1].contains("A") {
                        adjectives.push(parts[0].to_string());
                    }
                }
            }
        }
        let out_dir = "generated";
        write_word_file(format!("{}/nouns.txt", out_dir).as_str(), nouns).expect("failed to write nouns");
        write_word_file(format!("{}/adjectives.txt", out_dir).as_str(), adjectives).expect("failed to write adjectives");
    }
}

fn main() {
    generate_words();
}