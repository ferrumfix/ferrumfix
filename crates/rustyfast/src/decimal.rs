#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decimal {
    pub exponent: i32,
    pub mantissa: i64,
}

impl Decimal {
    pub fn new(exponent: i32, mantissa: i64) -> Self {
        Self { exponent, mantissa }
    }

    pub fn to_f64(&self) -> f64 {
        (self.mantissa as f64) * 10f64.powi(self.exponent)
    }
}
