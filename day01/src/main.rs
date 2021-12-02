use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use day1::depth_analyzer;

fn main() {
    first();
    second();
}

fn first() {
    println!("first answer is : {}", depth_analyzer::scan(load_data()));
}

fn second() {
    println!("second answer is : {}", depth_analyzer::scan(depth_analyzer::merge(load_data())));
}

fn load_data() -> Vec<u32>
{
    let mut datas = vec![];
    if let Ok(lines) = read_lines("./datas/first.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                datas.push(line.parse::<u32>().unwrap());
            }
        }
    }
    return datas;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
