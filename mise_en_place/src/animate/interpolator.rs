pub struct Interpolator {
    pub value: f32,
    total: f32,
    sign_positive: bool,
}

impl Interpolator {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            total: value,
            sign_positive: value.is_sign_positive(),
        }
    }
    pub fn extract(&mut self, delta: f32) -> (f32, bool) {
        let segment = self.total * delta;
        self.value -= segment;
        let overage = match self.sign_positive {
            true => {
                let mut val = None;
                if self.value.is_sign_negative() {
                    val = Some(self.value)
                }
                val
            }
            false => {
                let mut val = None;
                if self.value.is_sign_positive() {
                    val = Some(self.value)
                }
                val
            }
        };
        let mut extract = segment;
        let mut done = false;
        if let Some(over) = overage {
            extract += over;
            done = true;
        }
        if extract == 0.0 {
            done = true;
        }
        (extract, done)
    }
}
