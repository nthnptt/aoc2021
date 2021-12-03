use day2::position::Position;

#[test]
fn it_analyse_should_works() {
    let orders = vec![String::from("forward 3"), String::from("down 2"), String::from("up 4")];
    assert_eq!(Position{x: 3, y: -2}, day2::analyze(orders));
}
