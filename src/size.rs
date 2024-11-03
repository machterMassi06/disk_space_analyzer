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
        Self::new(self.0+other.0)
    }
}


#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    fn test_add_size(){
        let s1=Size::new(16);
        let s2=Size::new(64);
        let res = s1+s2;
        assert_eq!(res.0,80);
    }
} 