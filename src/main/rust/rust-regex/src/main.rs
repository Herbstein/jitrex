use regex::Regex;

const PATTERNS: &[&str] = &[
    "Twain",
    "(?i)Twain",
    "[a-z]shing",
    "Huck[a-zA-Z]+|Saw[a-zA-Z]+",
    "\\b\\w+nn\\b",
    "[a-q][^u-z]{13}x",
    "Tom|Sawyer|Huckleberry|Finn",
    "(?i)(Tom|Sawyer|Huckleberry|Finn)",
    ".{0,2}(?:Tom|Sawyer|Huckleberry|Finn)",
    ".{2,4}(Tom|Sawyer|Huckleberry|Finn)",
    "Tom.{10,25}river|river.{10,25}Tom",
    "[a-zA-Z]+ing",
    "\\s[a-zA-Z]{0,12}ing\\s",
    "([A-Za-z]awyer|[A-Za-z]inn)\\s",
    "[\"'][^\"']{0,30}[?!\\.][\"']",
    "\u{221E}|\u{2713}",
    "\\p{Sm}",
];

const INPUT_FILE: &str = include_str!("../data/3200.txt");

fn main() {
    let lines = INPUT_FILE.lines().collect::<Vec<_>>();

    println!("{:<40} | {:<10} | {:<10}", "pattern", "full", "lines");
    for pattern in PATTERNS {
        let regex = Regex::new(pattern).unwrap();

        let line_matches = lines.iter().filter(|l| regex.is_match(l)).count();
        let full_matches = regex.find_iter(INPUT_FILE).count();
        println!("{pattern:<40} | {full_matches:<10} | {line_matches}");
    }
}
