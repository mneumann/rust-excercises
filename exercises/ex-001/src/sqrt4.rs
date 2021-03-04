use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct NonNegativeFloat {
    num: f32
}

impl NonNegativeFloat {
    pub fn new(value: f32) -> NonNegativeFloat {
        if value < 0.0 {
            panic!("negative input")
        } else {
            NonNegativeFloat {
                num: value
            }
        }

    }

    /// Returns the encapsulated non-negative value
    pub fn value(&self) -> f32 {
        self.num
    }

    pub fn sqrt(&self) -> NonNegativeFloat {
        NonNegativeFloat {
            num: self.num.sqrt()
        }
    }
}

impl TryFrom<f32> for NonNegativeFloat {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 0.0 {
            Err("negative input".into())
        } else {
            Ok(
                NonNegativeFloat {
                    num: value
                }
            )
        }
    }
}
