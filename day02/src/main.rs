use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use day2::position::Position;

fn main() {
    first();
    second();
}

fn first() {
    println!("first answer is : {:?}", day2::move_no_aim(load_data()));
}

fn second() {
    let lines = load_data();
    let mut submarine = day2::Submarine::create(Position{x: 0, y: 0});

    for line in lines {
        submarine.execute_move(line);
    }

    println!("second answer is: {:?}", submarine);
}
fn load_data() -> Vec<String>
{
    let mut datas = vec![];
    if let Ok(lines) = read_lines("./datas/first.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                datas.push(line);
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
