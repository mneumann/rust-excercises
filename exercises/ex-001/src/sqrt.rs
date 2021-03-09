/// Square root function for Assignement 1
pub fn sqrt(input: f32) -> f32 {
    if input >= 0.0 {
        input.sqrt()
    } else {
        panic!("negative input");
    }
}
