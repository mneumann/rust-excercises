/// Square root function for Assignment 2
pub fn sqrt2(input: f32) -> Option<f32> {
    if input >= 0.0 {
        Some(input.sqrt())
    } else {
        None
    }
}
