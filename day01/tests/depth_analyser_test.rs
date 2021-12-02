use day1::depth_analyzer;
#[test]
fn it_should_compute_1_increase_from_1999_to_2000() {
    assert_eq!(1, depth_analyzer::scan(vec![1999, 2000]));
}

#[test]
fn it_should_compute_0_increase_from_2_to_1() {
    assert_eq!(0, depth_analyzer::scan(vec![2,1]));
}

#[test]
fn it_should_works_for_long_vec() {
    assert_eq!(4, depth_analyzer::scan(vec![2,1,2,3,2,3,4]));
}

#[test]
fn it_should_merge_array() {
    assert_eq!(vec![607], depth_analyzer::merge(vec![199, 200, 208]));
}
fn it_should_merge_work() {
    assert_eq!(vec![607, 618, 618, 617, 647, 716, 769, 792], depth_analyzer::merge(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
}
