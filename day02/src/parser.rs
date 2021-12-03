use crate::position::Position;

pub fn parse_line(line: String) -> Position {
    let mut split = line.split_whitespace().collect::<Vec<&str>>();
    let value = split[1].parse::<i32>().unwrap(); 
    let order = split[0]; 

    match order {
        "forward" => Position{x: value, y: 0},
        "up" => Position{x: 0, y: -1*value},
        "down" => Position{x: 0, y: 1*value},
        _ => Position{x: 0, y: 0},
    }
}
