use day9;

#[test]
fn it_should_extract_low_points() {
    let input: Vec<Vec<u32>> = vec![vec![2, 1, 4, 0], vec![1, 3, 5, 1], vec![3, 6, 2, 0]];
    assert_eq!(vec![1, 0, 1, 0], day9::extract_low_points(input));
}

#[test]
fn it_should_extract_low_points_hard() {
    let input: Vec<Vec<u32>> = vec![
        vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 3],
        vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ];
    assert_eq!(vec![1, 0, 5, 5], day9::extract_low_points(input));
}
