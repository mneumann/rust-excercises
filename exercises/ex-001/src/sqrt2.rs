/// Square root function for Assignement 2
pub fn sqrt2(input: f32) -> Option<f32> {
    if input < 0.0 {
        None
    } else {
        Some(input.sqrt())
    }
}
