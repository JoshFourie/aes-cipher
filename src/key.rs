use std::ops;
use crate::text;

pub struct Key([u8; 128]);

impl Key {
    fn new(seed: Seed) -> Self {
        unimplemented!()
    }
}

pub struct Seed;

impl Seed {
    fn new() -> Self {
        unimplemented!()
    }
}

impl ops::BitXor<text::Plain> for Key {
    type Output = text::Cipher;

    fn bitxor(self, rhs: text::Plain) -> Self::Output {
        unimplemented!()
    }
} 
