use yada::unit::*;

pub struct DoubleArrayParser<'a>(pub &'a [u8]);

impl DoubleArrayParser<'_> {
    pub fn inverse_da(&self) -> Vec<(String, u32)> {
        let Some(unit) = self.get_unit(0) else {
            return vec![];
        };
        self.dfs(0, unit.offset())
            .into_iter()
            .map(|(s, id)| (String::from_utf8(s).unwrap(), id))
            .collect()
    }

    fn dfs(&self, unit_pos: usize, unit_offset: u32) -> Vec<(Vec<u8>, u32)> {
        let mut keyset: Vec<(Vec<u8>, u32)> = vec![];
        for c in 1..256 {
            let node_pos = (unit_offset ^ unit_pos as u32 ^ c) as UnitID;

            match self.get_unit(node_pos) {
                Some(unit) if c == unit.label() => {
                    assert!(!unit.is_leaf());
                    if unit.has_leaf() {
                        let gc_pos = (unit.offset() ^ node_pos as u32) as UnitID;
                        let Some(gc_unit) = self.get_unit(gc_pos) else {
                            continue;
                        };
                        keyset.push((vec![c as u8], gc_unit.value()))
                    }
                    keyset.extend(self.dfs(node_pos, unit.offset()).into_iter().map(
                        |(mut s, id)| {
                            s.insert(0, c as u8);
                            (s, id)
                        },
                    ));
                }
                _ => continue,
            }
        }
        keyset
    }

    #[inline(always)]
    fn get_unit(&self, index: usize) -> Option<Unit> {
        let b = unsafe {
            // This unsafe method call does not lead unexpected transitions
            // when a double array was built properly.
            self.0
                .get_unchecked(index * UNIT_SIZE..(index + 1) * UNIT_SIZE)
        };
        match b.try_into() {
            Ok(bytes) => Some(Unit::from_u32(u32::from_le_bytes(bytes))),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use yada::builder::DoubleArrayBuilder;

    use crate::to_csv::da::DoubleArrayParser;

    #[test]
    fn test_build_search() {
        let keyset = &[
            ("a".as_bytes(), 0),
            ("ab".as_bytes(), 1),
            ("aba".as_bytes(), 2),
            ("ac".as_bytes(), 3),
            ("acb".as_bytes(), 4),
            ("acc".as_bytes(), 5),
            ("ad".as_bytes(), 6),
            ("ba".as_bytes(), 7),
            ("bb".as_bytes(), 8),
            ("bc".as_bytes(), 9),
            ("c".as_bytes(), 10),
            ("caa".as_bytes(), 11),
        ];

        let da_bytes = DoubleArrayBuilder::build(keyset);
        assert!(da_bytes.is_some());

        let mut inverse = DoubleArrayParser(da_bytes.as_ref().unwrap()).inverse_da();
        inverse.sort_by_key(|(_, id)| *id);
        for (n, (s, id)) in inverse.iter().enumerate() {
            assert_eq!(keyset[n].0, s.as_bytes());
            assert_eq!(keyset[n].1, *id);
        }
    }
}
