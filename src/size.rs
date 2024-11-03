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
        let value = self.0 as f64;
        // formattons la valeur a une unite adaptée
        let (formatted_value,suffix)= match value {
            1_048_576.0 .. => (value /1_048_576.0,"MiB"),
            1_024.0 .. => (value/1_024.0,"KiB"),
            _ => (value,"bytes"), 
        };
        // Affiche le résultat avec une précision de 1 décimale
        write!(f, "{:.1} {}", formatted_value, suffix)
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
    #[test]
    fn test_display_bytes(){
        let size=Size::new(500);
        assert_eq!(format!("{}",size),"500.0 bytes");
    }
    #[test]
    fn test_display_kib(){
        let size=Size::new(1024);
        assert_eq!(format!("{}",size),"1.0 KiB");
    }
    #[test]
    fn test_display_mib(){
        let size=Size::new(2411724);
        assert_eq!(format!("{}",size),"2.3 MiB");
    }
} 