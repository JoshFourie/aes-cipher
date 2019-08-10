use std::ops;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Byte(u8);

impl Byte {
    pub fn zero() -> Self {
        Byte(0)
    }

    pub fn as_inner(self) -> u8 {
        self.0
    }
}

impl PartialEq<u8> for Byte {
    fn eq(&self, rhs: &u8) -> bool {
        self.0 == *rhs
    }
}

impl PartialEq<Byte> for u8 {
    fn eq(&self, rhs: &Byte) -> bool {
        rhs.0 == *self
    }
}

impl From<u8> for Byte {
    fn from(inner: u8) -> Self {
        Byte(inner)
    }
}

impl ops::BitXor<Self> for Byte {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from(self.0 ^ rhs.0)
    }
}

impl ops::Sub<Self> for Byte {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self ^ rhs
    }
}   

impl ops::Add<Self> for Byte {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self ^ rhs
    }
}

impl ops::Add<u8> for Byte {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Self::from(self.0 ^ rhs)
    }
}

impl ops::Mul<Self> for Byte {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl ops::Mul<u8> for Byte {
    type Output = Self;
    fn mul(self, rhs: u8) -> Self::Output {
        Self::from(self.0.wrapping_mul(rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests from https://sites.math.washington.edu/~morrow/336_12/papers/juan.pdf
    // addition is incorrect in that document
    #[test]
    fn test_add() {
        let test: Byte = Byte::from(83) + Byte::from(249);
        let exp: _ = Byte::from(170);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_mul() {
        let test: Byte = Byte::from(0x53) * Byte::from(0xca);
        let exp: _ = Byte::from(0x01);
        assert_eq!(test, exp);
    }
}
