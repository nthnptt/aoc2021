use day10::Validation;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    first();
    second();
}

fn first() {
    let output: u32 = load_data()
        .into_iter()
        .filter_map(|line| match day10::check(line) {
            Validation::Corrupted(char) => Some(char),
            _ => None,
        })
        .fold(0, |acc, char| acc + day10::to_score(char));

    println!("part I : {:?}", output);
}

fn second() {
    let mut output: Vec<u128> = load_data()
        .into_iter()
        .filter(|line| day10::check(line.to_string()) == Validation::Incomplete)
        .map(|line| day10::repair_to_score(day10::repair(line)))
        .collect();
    output.sort();
    let size = output.len();
    let result_index = if size % 2 == 0 {
        (size / 2) - 1
    } else {
        (size - 1) / 2
    };

    println!("part II : {:?}", output[result_index]);
}

fn load_data() -> Vec<String> {
    let mut datas = vec![];
    if let Ok(lines) = read_lines("./datas/first.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                datas.push(line.to_string());
            }
        }
    }
    return datas;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
