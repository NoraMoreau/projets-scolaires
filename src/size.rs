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
        
        let valeur: f64;
        if self.0 > 999 && self.0 < 10000 {
            valeur = self.0 as f64 / 1000.0;
            return write!(f, "{:.1}KB", valeur); // :.1 met un seul chiffre après la virgule

        }else if self.0 > 9999 && self.0 < 100000 {
            valeur = self.0 as f64 / 10000.0;
            return write!(f, "{:.1}MB", valeur); // :.1 met un seul chiffre après la virgule

        } else if self.0 > 99999 && self.0 < 1000000 {
            valeur = self.0 as f64 / 100000.0;
            return write!(f, "{:.1}GB", valeur); // :.1 met un seul chiffre après la virgule

        }else {
            valeur = self.0 as f64;
        }

        write!(f, "{:.1}", valeur) // :.1 met un seul chiffre après la virgule
    }
}

impl std::ops::Add for Size { //pour tester add on utilise pas add() mais +
    type Output = Self; //le résultat de l'addition de deux valeurs est de type Size
    fn add(self, other: Self) -> Self::Output {
        Size(self.0 + other.0) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = Size::new(1024);
        println!("s = {}", s);
        assert_eq!(s.0, 1024);
    }

    #[test]
    fn test_add() {
        let s1 = Size::new(1064);
        let s2 = Size::new(1000);
        print!("add(s1, s2) = {}", s1 + s2);
        assert_eq!(2064, s1.0 + s2.0);
    }

    #[test]
    fn test_fmt_1_000() {
        let s1 = Size::new(1024);
        println!("Formatted result: {}", s1);
        let s2 = Size::new(4674);
        println!("Formatted result: {}", s2);
    }

    #[test]
    fn test_fmt_10_000() {
        let s1 = Size::new(11024);
        println!("Formatted result: {}", s1);
        let s2 = Size::new(47674);
        println!("Formatted result: {}", s2);
    }

    #[test]
    fn test_fmt_100_000() {
        let s1 = Size::new(121024);
        println!("Formatted result: {}", s1);
        let s2 = Size::new(497674);
        println!("Formatted result: {}", s2);
    }
}
