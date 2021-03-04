/// Square root function for Assignement 3
pub fn sqrt3(input : f32) -> Result<f32, String> {
    if input < 0.0 {
        Err("negative input".into())
    } else {
        Ok(input.sqrt())
    }
}
