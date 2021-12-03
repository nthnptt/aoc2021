pub mod position;
pub mod parser;

use position::Position;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Submarine {
    pub position: Position,
    pub aim: i32,
}

impl Submarine {
    pub fn create(position: Position) -> Self {
        Submarine{position: position, aim: 0}
    }

    pub fn execute_move(&mut self, order: String){
        let split = order.split_whitespace().collect::<Vec<&str>>();
        let value = split[1].parse::<i32>().unwrap(); 
        let order = split[0]; 

        let position = match order {
            "forward" => Position{x: value, y: self.aim * value},
            "down" => {
                self.aim += value;
                Position{x: 0, y: 0}
            },
            "up" => {
                self.aim -= value;
                Position{x: 0, y: 0}
            },
            _ => Position{x: 0, y: 0},
        };

        self.position = self.position + position;
    }
}

pub fn move_no_aim(lines: Vec<String>) -> Position {
    let result = lines.iter()
        .map(|line| { parser::parse_line(line.clone()) })
        .reduce(|a, b| { a + b });


    match result {
       Some(pos) => pos,
       _ => Position{x: 0, y: 0}
    }
}
