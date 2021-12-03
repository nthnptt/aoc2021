use day2::position::Position;

#[test]
fn it_shoud_sum_two_position() {
    assert_eq!(Position {x: 3, y: 3}, Position {x: 1, y: 0} + Position {x: 2, y: 3});
}
