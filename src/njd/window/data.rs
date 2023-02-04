#[derive(PartialEq, Debug)]
pub enum Quintuple<T> {
    /// None, Some(T), None, None, None
    Single(T),
    /// None, Some(T), Some(T), None, None
    Double(T, T),
    /// None, Some(T), Some(T), Some(T), None
    Triple(T, T, T),
    /// None, Some(T), Some(T), Some(T), Some(T)
    First(T, T, T, T),
    /// Some(T), Some(T), Some(T), Some(T), Some(T)
    Full(T, T, T, T, T),
    /// Some(T), Some(T), Some(T), Some(T), None
    ThreeLeft(T, T, T, T),
    /// Some(T), Some(T), Some(T), None, None
    TwoLeft(T, T, T),
    /// Some(T), Some(T), None, None, None
    Last(T, T),
}

#[derive(PartialEq, Debug)]
pub enum Triple<T> {
    /// None, Some(T), None
    Single(T),
    /// None, Some(T), Some(T)
    First(T, T),
    /// Some(T), Some(T), Some(T)
    Full(T, T, T),
    /// Some(T), Some(T), None
    Last(T, T),
}

impl<T> From<Quintuple<T>> for Triple<T> {
    fn from(value: Quintuple<T>) -> Self {
        match value {
            Quintuple::Single(c) => Self::Single(c),
            Quintuple::Double(c, nx1) => Self::First(c, nx1),
            Quintuple::Triple(c, nx1, _nx2) => Self::First(c, nx1),
            Quintuple::First(c, nx1, _nx2, _nx3) => Self::First(c, nx1),
            Quintuple::Full(p, c, nx1, _nx2, _nx3) => Self::Full(p, c, nx1),
            Quintuple::ThreeLeft(p, c, nx1, _nx2) => Self::Full(p, c, nx1),
            Quintuple::TwoLeft(p, c, nx1) => Self::Full(p, c, nx1),
            Quintuple::Last(p, c) => Self::Last(p, c),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Double<T> {
    /// None, Some(T)
    First(T),
    /// Some(T), Some(T)
    Full(T, T),
}

impl<T> From<Quintuple<T>> for Double<T> {
    fn from(value: Quintuple<T>) -> Self {
        match value {
            Quintuple::Single(c) => Self::First(c),
            Quintuple::Double(c, _nx1) => Self::First(c),
            Quintuple::Triple(c, _nx1, _nx2) => Self::First(c),
            Quintuple::First(c, _nx1, _nx2, _nx3) => Self::First(c),
            Quintuple::Full(p, c, _nx1, _nx2, _nx3) => Self::Full(p, c),
            Quintuple::ThreeLeft(p, c, _nx1, _nx2) => Self::Full(p, c),
            Quintuple::TwoLeft(p, c, _nx1) => Self::Full(p, c),
            Quintuple::Last(p, c) => Self::Full(p, c),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum QuadForward<T> {
    /// Some(T), None, None, None
    Single(T),
    /// Some(T), Some(T), None, None
    Double(T, T),
    /// Some(T), Some(T), Some(T), None
    Triple(T, T, T),
    /// Some(T), Some(T), Some(T), Some(T)
    Full(T, T, T, T),
}

impl<T> From<Quintuple<T>> for QuadForward<T> {
    fn from(value: Quintuple<T>) -> Self {
        match value {
            Quintuple::Single(c) => Self::Single(c),
            Quintuple::Double(c, nx1) => Self::Double(c, nx1),
            Quintuple::Triple(c, nx1, nx2) => Self::Triple(c, nx1, nx2),
            Quintuple::First(c, nx1, nx2, nx3) => Self::Full(c, nx1, nx2, nx3),
            Quintuple::Full(_p, c, nx1, nx2, nx3) => Self::Full(c, nx1, nx2, nx3),
            Quintuple::ThreeLeft(_p, c, nx1, nx2) => Self::Triple(c, nx1, nx2),
            Quintuple::TwoLeft(_p, c, nx1) => Self::Double(c, nx1),
            Quintuple::Last(_p, c) => Self::Single(c),
        }
    }
}
