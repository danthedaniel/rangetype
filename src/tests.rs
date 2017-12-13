use super::RangeType;

#[test]
fn test_integer() {
    range!(1, 0..10);
}

#[test]
fn test_float() {
    range!(1.0, 0.0..10.0);
}

#[test]
#[should_panic]
fn test_addition_diff_range() {
    range!(1, 0..1) + range!(1, 0..2);
}

#[test]
fn test_eq() {
    assert!(range!(1, 0..1) == range!(1, 0..1));
}

#[test]
#[should_panic]
fn test_neq() {
    assert!(range!(1, 0..1) != range!(1, 0..1));
}

#[test]
fn test_neq_range() {
    assert!(range!(1, 0..1) != range!(1, 0..3));
}

#[test]
fn test_less_than() {
    assert!(range!(1, 0..3) < range!(2, 1..4));
}
