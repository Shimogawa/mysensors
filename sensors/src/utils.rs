pub(crate) struct FixedF8 {
    integer: u8,
    fractional: u8,
    is_neg: bool,
}

impl FixedF8 {
    pub fn new(integer: u8, fractional: u8, is_neg: bool) -> Self {
        Self {
            integer,
            fractional,
            is_neg,
        }
    }

    pub fn to_f32(&self) -> f32 {
        let res = self.integer as f32 + self.fractional as f32 / 256.0;
        if self.is_neg {
            -res
        } else {
            res
        }
    }
}

impl From<f32> for FixedF8 {
    fn from(value: f32) -> Self {
        let is_neg = value < 0.0;
        let value = value.abs();
        let integer = value.trunc() as u8;
        let fractional = (value.fract() * 256.0).round() as u8;
        Self::new(integer, fractional, is_neg)
    }
}

impl From<FixedF8> for f32 {
    fn from(value: FixedF8) -> Self {
        value.to_f32()
    }
}
