#[test]
fn sqrt3_is_correct_for_positive_value() {
    assert_eq!(Ok(4.0), ex_001::sqrt3(16.0));
}

#[test]
fn sqrt3_returns_zero_given_zero_input() {
    assert_eq!(Ok(0.0), ex_001::sqrt3(0.0));
}

#[test]
fn sqrt3_is_partial_for_negative_input() {
    assert!(ex_001::sqrt3(-1.0).is_err());
}
