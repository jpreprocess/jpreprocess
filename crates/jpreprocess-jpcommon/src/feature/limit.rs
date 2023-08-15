pub enum Limit {
    /// unsigned: 1 - 19, signed: -19 - 19
    S,
    /// unsigned: 1 - 49, signed: -49 - 49
    M,
    /// unsigned: 1 - 99, signed: -99 - 99
    L,
    /// unsigned: 1 - 199, signed: -199 - 199
    LL,
}
impl Limit {
    pub fn ulimit(self, value: usize) -> usize {
        match self {
            Self::S => Self::clamp(value, 1, 19),
            Self::M => Self::clamp(value, 1, 49),
            Self::L => Self::clamp(value, 1, 99),
            Self::LL => Self::clamp(value, 1, 199),
        }
    }
    pub fn ilimit(self, value: isize) -> isize {
        match self {
            Self::S => Self::clamp(value, -19, 19),
            Self::M => Self::clamp(value, -49, 49),
            Self::L => Self::clamp(value, -99, 99),
            Self::LL => Self::clamp(value, -199, 199),
        }
    }
    fn clamp<T>(value: T, lower_bound: T, upper_bound: T) -> T
    where
        T: PartialOrd,
    {
        if value < lower_bound {
            lower_bound
        } else if upper_bound < value {
            upper_bound
        } else {
            value
        }
    }
}
