pub mod position;
pub mod parser;

use position::Position;

pub fn analyze(lines: Vec<String>) -> Position {
    let result = lines.iter()
        .map(|line| { parser::parse_line(line.clone()) })
        .reduce(|a, b| { a + b });


    match result {
       Some(pos) => pos,
       _ => Position{x: 0, y: 0}
    }
}
