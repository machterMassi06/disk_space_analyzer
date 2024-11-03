use std::fmt;

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct Size(u64);

impl Size {
    pub fn new(bytes: u64) -> Self {
        Self(bytes)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add for Size {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        unimplemented!()
    }
}
