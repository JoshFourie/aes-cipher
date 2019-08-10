use std::ops;
use crate::key;

pub struct Cipher([u8; 128]);

pub struct Plain([u8; 128]);

impl ops::BitXor<key::Key> for Plain {
    type Output = Self;

    fn bitxor(self, rhs: key::Key) -> Plain {
        unimplemented!()
    }
}
