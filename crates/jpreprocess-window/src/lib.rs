//! An iterator-like object which returns contiguous windows containing five mutable references.
//!
//! ## Example
//! ```rust
//! use jpreprocess_window::*;
//!
//! let mut vector = [0, 1, 2, 3, 4];
//! let mut iter = IterQuintMut::new(&mut vector);
//! assert_eq!(iter.next().unwrap(), Quintuple::First(&mut 0, &mut 1, &mut 2, &mut 3));
//! assert_eq!(iter.next().unwrap(), Quintuple::Full(&mut 0, &mut 1, &mut 2, &mut 3, &mut 4));
//! assert_eq!(iter.next().unwrap(), Quintuple::ThreeLeft(&mut 1, &mut 2, &mut 3, &mut 4));
//! assert_eq!(iter.next().unwrap(), Quintuple::TwoLeft(&mut 2, &mut 3, &mut 4));
//! assert_eq!(iter.next().unwrap(), Quintuple::Last(&mut 3, &mut 4));
//! ```

pub mod structures;
pub use structures::*;

pub trait IterQuintMutTrait {
    type Item;
    fn iter_quint_mut(&mut self) -> IterQuintMut<'_, Self::Item>;
    fn iter_quint_mut_range(&mut self, start: usize, end: usize) -> IterQuintMut<'_, Self::Item>;
}

pub struct IterQuintMut<'a, T> {
    vec: &'a mut [T],
    target: usize,
}

impl<'a, T> IterQuintMut<'a, T> {
    pub fn new(vec: &'a mut [T]) -> Self {
        Self { vec, target: 0 }
    }

    // This method cannot be converted into trait because of the mutable references
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Quintuple<&mut T>> {
        let next = Self::next_iter(self.target, self.vec);
        self.target += 1;
        next
    }

    fn next_iter(target: usize, vec: &mut [T]) -> Option<Quintuple<&mut T>> {
        use Quintuple::*;
        match (vec.len(), target) {
            (0, _) => None,
            (1, 0) => vec.first_mut().map(Single),
            (1, _) => None,
            (2, 0) => {
                let [i0, i1] = &mut vec[0..2] else {
                    unreachable!()
                };
                Some(Double(i0, i1))
            }
            (3, 0) => {
                let [i0, i1, i2] = &mut vec[0..3] else {
                    unreachable!()
                };
                Some(Triple(i0, i1, i2))
            }
            (_, 0) => {
                let [i0, i1, i2, i3] = &mut vec[0..4] else {
                    unreachable!()
                };
                Some(First(i0, i1, i2, i3))
            }
            (_, t) => match &mut vec[t - 1..] {
                [i0, i1, i2, i3, i4, ..] => Some(Full(i0, i1, i2, i3, i4)),
                [i0, i1, i2, i3] => Some(ThreeLeft(i0, i1, i2, i3)),
                [i0, i1, i2] => Some(TwoLeft(i0, i1, i2)),
                [i0, i1] => Some(Last(i0, i1)),
                [_] => None,
                _ => unreachable!(),
            },
        }
    }
}
