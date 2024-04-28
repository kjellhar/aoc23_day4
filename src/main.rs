use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FILENAME: &str = "input.txt";
//const FILENAME: &str = "test.txt";

fn main() {

    let mut win_points = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(FILENAME) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let split_line = line.split([':', '|']).collect::<Vec<&str>>();

            let winning_number = split_line[1]
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let ticket_number = split_line[2]
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
               .iter()
               .map(|x| x.trim().parse::<usize>().unwrap())
               .collect::<Vec<usize>>();

            let win_count = ticket_number
                .iter()
                .filter(|&x| winning_number.contains(x))
                .count();

            if win_count > 0 {
                win_points += 2_usize.pow((win_count-1) as u32);
            }
            

        }
    }

    println!("Winning points: {}", win_points);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
