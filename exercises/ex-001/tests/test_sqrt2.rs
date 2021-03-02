#[test]
fn sqrt2_is_correct_for_positive_value() {
    assert_eq!(Some(4.0), ex_001::sqrt2(16.0));
}

#[test]
fn sqrt2_returns_zero_given_zero_input() {
    assert_eq!(Some(0.0), ex_001::sqrt2(0.0));
}

#[test]
fn sqrt2_is_partial_for_negative_input() {
    assert_eq!(None, ex_001::sqrt2(-1.0));
}
