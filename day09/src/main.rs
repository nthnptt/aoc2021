use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let result: u32 = day9::extract_low_points(load_data())
        .iter()
        .map(|n| n + 1)
        .sum();
    println!("{}", result);
}

fn first() {}

fn load_data() -> Vec<Vec<u32>> {
    let mut datas = vec![];
    if let Ok(lines) = read_lines("./datas/first.txt") {
        //    if let Ok(lines) = read_lines("./datas/example.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let mut row = vec![];
                for chr in line.chars() {
                    match chr.to_digit(10) {
                        Some(number) => row.push(number),
                        None => (),
                    }
                }
                datas.push(row);
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
