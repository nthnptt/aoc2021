use day2::position::Position;

#[test]
fn it_move_no_aim_should_works() {
    let orders = vec![String::from("forward 3"), String::from("down 2"), String::from("up 4")];
    assert_eq!(Position{x: 3, y: -2}, day2::move_no_aim(orders));
}

#[test]
fn it_should_init_submarine_with_pos() {
    let submarine = day2::Submarine::create(Position{x: 1, y: 2});
    assert_eq!(Position{x: 1, y: 2}, submarine.position);
}

#[test]
fn it_should_init_submarine_with_aim_zero() {
    let submarine = day2::Submarine::create(Position{x: 1, y: 2});
    assert_eq!(0, submarine.aim);
}

#[test]
fn down_2_to_submarine_should_add_to_aim_2() {
    let mut submarine = day2::Submarine::create(Position{x: 0, y: 0});
    submarine.execute_move(String::from("down 2"));
    assert_eq!(2, submarine.aim);
}


#[test]
fn down_3_to_submarine_with_2_aim_should_add_to_aim_3() {
    let mut submarine = day2::Submarine::create(Position{x: 0, y: 0});
    submarine.execute_move(String::from("down 2"));
    submarine.execute_move(String::from("down 3"));
    assert_eq!(5, submarine.aim);
}

#[test]
fn up_2_to_submarine_should_remove_to_aim_2() {
    let mut submarine = day2::Submarine::create(Position{x: 0, y: 0});
    submarine.execute_move(String::from("down 4"));
    submarine.execute_move(String::from("up 2"));
    assert_eq!(2, submarine.aim);
}

#[test]
fn forward_2_to_submarine_should_move_to_x_2() {
    let mut submarine = day2::Submarine::create(Position{x: 0, y: 0});
    submarine.execute_move(String::from("forward 2"));
    assert_eq!(2, submarine.position.x);
}

#[test]
fn forward_2_to_submarine_should_move_to_y() {
    let mut submarine = day2::Submarine::create(Position{x: 0, y: 0});
    submarine.execute_move(String::from("down 2"));
    submarine.execute_move(String::from("forward 2"));
    assert_eq!(Position{x: 2, y: 4}, submarine.position);
}
