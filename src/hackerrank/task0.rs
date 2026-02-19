// https://www.hackerrank.com/challenges/simple-array-sum/problem
fn simple_array_sum(aa: &[i32]) -> i32 {
    let mut x: i32 = 0;
    for a in aa {
        x += a
    }
    x
}

#[test]
fn test0() {
    let aa = vec![10, 11, 12];
    let real = simple_array_sum(&aa);
    let expected = 10 + 11 + 12;
    assert_eq!(real, expected);
}
