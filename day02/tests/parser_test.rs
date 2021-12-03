use day2::position::Position;
use day2::parser;

#[test]
fn it_shoud_parse_forward_line() {
    assert_eq!(Position{x: 3, y: 0}, parser::parse_line(String::from("forward 3")));
}

#[test]
fn it_shoud_parse_down_line() {
    assert_eq!(Position{x: 0, y: 1}, parser::parse_line(String::from("down 1")));
}

#[test]
fn it_shoud_parse_top_line() {
    assert_eq!(Position{x: 0, y: -1}, parser::parse_line(String::from("up 1")));
}
