pub mod data;
pub use data::*;

pub trait IterQuintMutTrait {
    type Item;
    fn iter_quint_mut(&mut self) -> IterQuintMut<'_, Self::Item>;
    fn iter_quint_mut_range(
        &mut self,
        start: usize,
        end: usize,
    ) -> IterQuintMut<'_, Self::Item>;
}

pub struct IterQuintMut<'a, T> {
    vec: &'a mut [T],
    target: usize,
}

impl<'a, T> IterQuintMut<'a, T> {
    pub fn new(vec: &'a mut [T]) -> Self {
        Self { vec, target: 0 }
    }

    pub fn next(&mut self) -> Option<Quintuple<&mut T>> {
        let next = Self::next_iter(self.target, self.vec);
        self.target += 1;
        next
    }

    // pub fn next_with_extra(
    //     &mut self,
    //     index: Option<usize>,
    // ) -> (usize, Option<&mut T>, Option<Quintuple<&mut T>>) {
    //     if let Some(i) = index {
    //         if i + 1 >= self.target {
    //             panic!()
    //         }
    //         // self.target>=1 is guaranteed by previous `if`
    //         let (a, b) = self.vec.split_at_mut(self.target - 1);

    //         let next = Self::next_iter(1, b);
    //         let result = (self.target, a.get_mut(i), next);
    //         self.target += 1;
    //         result
    //     } else {
    //         (self.target, None, self.next())
    //     }
    // }

    fn next_iter(target: usize, vec: &mut [T]) -> Option<Quintuple<&mut T>> {
        use Quintuple::*;
        match (vec.len(), target) {
            (0, _) => None,
            (1, 0) => vec.first_mut().map(Single),
            (1, _) => None,
            (2, 0) => {
                let [i0, i1] = &mut vec[0..2] else { unreachable!() };
                Some(Double(i0, i1))
            }
            (3, 0) => {
                let [i0, i1, i2] = &mut vec[0..3] else { unreachable!() };
                Some(Triple(i0, i1, i2))
            }
            (_, 0) => {
                let [i0, i1, i2,i3] = &mut vec[0..4] else { unreachable!() };
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
