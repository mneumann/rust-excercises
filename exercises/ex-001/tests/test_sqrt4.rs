use ex_001::NonNegativeFloat;
use std::convert::TryFrom;

#[test]
fn can_construct_non_negative_float_from_non_negative_value() {
    assert_eq!(16.0, NonNegativeFloat::new(16.0).value());
    assert_eq!(0.0, NonNegativeFloat::new(0.0).value());
}

#[test]
#[should_panic(expected = "negative input")]
fn non_negative_float_fails_to_construct_given_negative_value() {
    let _ = NonNegativeFloat::new(-1.0);
}

#[test]
fn non_negative_float_calculates_sqrt_correctly() {
    assert_eq!(4.0, NonNegativeFloat::new(16.0).sqrt().value());
}

#[test]
fn can_construct_non_negative_float_from_non_negative_value_with_try_from() {
    assert_eq!(Ok(NonNegativeFloat::new(16.0)), NonNegativeFloat::try_from(16.0));
}

#[test]
fn non_negative_float_fails_to_construct_given_negative_value_with_try_from() {
    assert_eq!(Err("negative input".into()), NonNegativeFloat::try_from(-1.0));
}
