pub struct NonNegativeFloat {
    value: f32,
}

impl NonNegativeFloat {
    pub fn new(value: f32) -> NonNegativeFloat {
        if value >= 0.0 {
            Self { value }
        } else {
            panic!("negative input");
        }
    }

    /// Returns the encapsulated non-negative value
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn sqrt(&self) -> NonNegativeFloat {
        Self {
            value: self.value.sqrt(),
        }
    }
}
