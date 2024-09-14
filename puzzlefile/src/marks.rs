use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Marks([bool; 9]);
impl Marks {
    pub(crate) fn into_bytes(&self) -> u16 {
        let mut mask = 0;

        for i in 0..9 {
            mask <<= 1;
            if self.0[i] {
                mask |= 1;
            }
        }

        mask
    }

    pub(crate) fn from_bytes(bytes: u16) -> Self {
        let mut flags = [false; 9];
        let mut mask = 1;

        for i in (0..9).rev() {
            flags[i] = mask & bytes > 0;
            mask <<= 1;
        }

        Self(flags)
    }

    pub fn marks(&self) -> Vec<u8> {
        (0..9)
            .rev()
            .filter_map(|i| if self.0[i] { Some((8 - i) as u8) } else { None })
            .collect()
    }

    pub fn from_marks<I: Iterator<Item = u8>>(marks: I) -> Self {
        let mut mask = 0;

        for mark in marks {
            mask |= 1 << mark;
        }

        return Self::from_bytes(mask);
    }

    pub fn unset(&self) -> bool {
        self.0.iter().any(|f| *f)
    }
}
impl Default for Marks {
    fn default() -> Self {
        Self([false; 9])
    }
}

#[cfg(test)]
mod tests {
    use super::Marks;
    #[test]
    fn test_pencilmarks() {
        let mask: u16 = 0b011010010;

        let marks = Marks::from_bytes(mask);
        assert_eq!(marks.into_bytes(), mask);
    }

    #[test]
    fn test_from_marks() {
        let expected_marks = vec![2, 5, 7, 8];
        let marks = Marks::from_marks(expected_marks.iter().copied());
        assert_eq!(marks.marks(), expected_marks);
    }
}
