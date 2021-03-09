/// Square root function for Assignment 3
pub fn sqrt3(input: f32) -> Result<f32, &'static str> {
    if input >= 0.0 {
        Ok(input.sqrt())
    } else {
        Err("Square-root of negative number does not exist")
    }
}
