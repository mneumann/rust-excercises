#[test]
fn sqrt_is_correct_for_positive_value() {
    assert_eq!(4.0, ex_001::sqrt(16.0));
}

#[test]
fn sqrt_returns_zero_given_zero_input() {
    assert_eq!(0.0, ex_001::sqrt(0.0));
}

#[test]
#[should_panic(expected = "negative input")]
fn sqrt_should_panic_given_negative_input() {
    let _ = ex_001::sqrt(-1.0);
}
