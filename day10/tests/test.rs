use day10;

#[test]
fn it_should_check_file_is_not_corrupted() {
    assert_eq!(
        day10::Validation::Valid,
        day10::check(String::from("([{()}])"))
    );
}

#[test]
fn it_should_check_input_is_corrupted() {
    assert_eq!(
        day10::Validation::Corrupted('}'),
        day10::check(String::from("(}"))
    );
    assert_eq!(
        day10::Validation::Corrupted('}'),
        day10::check("{([(<{}[<>[]}>{[]{[(<()>".to_string())
    );
}

#[test]
fn it_should_check_input_is_incomplete() {
    assert_eq!(
        day10::Validation::Incomplete,
        day10::check(String::from("([<()>]"))
    );
}

#[test]
fn it_should_repair_incomplete_string() {
    assert_eq!("}>])", day10::repair("([<{".to_string()));
}

#[test]
fn it_should_compute_good_score() {
    assert_eq!(294, day10::repair_to_score("])}>".to_string()));
    assert_eq!(288957, day10::repair_to_score("}}]])})]".to_string()));
}
