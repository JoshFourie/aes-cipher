use std::{slice, ops,iter};

use crate::primitives::{sbox, byte};

#[derive(Clone, Debug, PartialEq)]
pub struct Word([byte::Byte; 4]);

impl Word {
    pub fn iter_mut(&mut self) -> slice::IterMut<'_,byte::Byte> {
        self.0.iter_mut()
    }
}

impl Word {
    pub fn rotate(mut self) -> Self {
        self << 1
    }

    pub fn substitute(mut self, sbox: &sbox::SubBox) -> Self {
        for val in self.iter_mut() {
            *val = sbox.substitute(*val)
        }
        self
    }

    pub fn clone_from_slice(&mut self, slice: &[byte::Byte]) {
        self.0.clone_from_slice(slice)
    } 
}

impl ops::Shl<usize> for Word {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            let buf: _ = self[0];
            self[0] = self[1];
            self[1] = self[2];
            self[2] = self[3];
            self[3] = buf;
        }
        self 
    }
}

impl ops::BitXor<Self> for Word {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs)
            .map(|(lhs,rhs)| lhs ^ rhs)
            .collect()
    }
}

impl ops::Index<usize> for Word {
    type Output = byte::Byte;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl ops::IndexMut<usize> for Word {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl IntoIterator for Word {
    type Item = byte::Byte;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_vec().into_iter()
    }
}

impl iter::FromIterator<byte::Byte> for Word {
    fn from_iter<I: IntoIterator<Item=byte::Byte>>(object: I) -> Self {
        let mut iter: _ = object.into_iter();
        let arr: [byte::Byte; 4] = [
            iter.next().expect("attempting to collect a Byte from a malformed iterator: required 4 additional elements."), 
            iter.next().expect("attempting to collect a Byte from a malformed iterator: required 3 additional elements."), 
            iter.next().expect("attempting to collect a Byte from a malformed iterator: required 2 additional elements."), 
            iter.next().expect("attempting to collect a Byte from a malformed iterator: required 1 additional elements.")
        ];
        if iter.next().is_some() {
            let (_, upper): _ = iter.size_hint();
            let remaining_elements: _ = upper.unwrap();
            panic!("attempting to collect a Byte from a malformed iterator: there are {} remaining elements.", remaining_elements+1)
        } else {
            Self::from(arr)
        }
    }
}

impl From<[u8; 4]> for Word {
    fn from(inner: [u8; 4]) -> Self {
        let buf: [byte::Byte; 4] = [
            inner[0].into(),
            inner[1].into(),
            inner[2].into(),
            inner[3].into()
        ];
        Word(buf)
    }
}

impl From<[byte::Byte; 4]> for Word {
    fn from(inner: [byte::Byte; 4]) -> Self {
        Word(inner)
    }
}

impl Default for Word {
    fn default() -> Self {
        Self::from([0,0,0,0])
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_substitute() {
        let sbox: _ = crate::primitives::sbox::SubBox::default();
        let test: _ = Word::from([0x7f, 0x67, 0x98, 0xaf]).substitute(&sbox);
        let exp: _ = Word::from([0xd2, 0x85, 0x46, 0x79]);
        assert_eq!(test, exp)        
    }

    #[test]
    fn test_bitxor() {
        let word_a: _ = Word::from([0,1,2,3]);
        let word_b: _ = Word::from([1,2,3,4]);
        let test_word: _ = word_a ^ word_b;
        let exp_word: _ =  Word::from([1,3,1,7]);

        assert_eq!(test_word, exp_word)
    }

    #[test]
    fn test_rotate() {
        let test: _ = Word::from([0xaf, 0x7f, 0x67, 0x98]);
        let exp: _ = Word::from([0x7f, 0x67, 0x98, 0xaf]);
        assert_eq!(test.rotate(), exp)
    }

    #[test]
    fn test_shl() {
        let test: _ = Word::from([0xaf, 0x7f, 0x67, 0x98]);
        let exp: _ = Word::from([0x67, 0x98, 0xaf, 0x7f]);
        assert_eq!(test << 2, exp)
    }
}
