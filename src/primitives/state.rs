use std::{iter,ops};
use crate::primitives::{byte, word};

#[derive(Debug, Clone, PartialEq)]
pub struct State([word::Word; 4]);

impl State {
    pub fn col(&self, idx: usize) -> word::Word {
        let mut buf: [byte::Byte; 4] = [byte::Byte::from(0); 4];
        for i in 0..4 {
            buf[i] = self[i][idx]
        }
        word::Word::from(buf)
    }

    pub fn row(&self, idx: usize) -> word::Word {
        self[idx].clone()
    }

    pub fn into_row_iter(self) -> impl Iterator<Item=byte::Byte> {
        self.transpose()
            .into_iter()
            .flat_map(|x| x.clone().into_iter())
    }

    pub fn into_col_iter(self) -> impl Iterator<Item=byte::Byte> {
        self.into_iter()
            .flat_map(|x| x.clone().into_iter())
    }

    pub fn into_rows(self) -> impl Iterator<Item=word::Word> {
        self.into_iter()
    }

    pub fn into_cols(self) -> impl Iterator<Item=word::Word> {
        self.transpose().into_iter()
    }

    pub fn map_to_col<F>(&mut self, row: usize, op: F) -> &Self 
    where
        F: Fn(byte::Byte) -> byte::Byte
    {
        for col in 0..4 {
            self[col][row] = op(self[col][row])
        }
        self
    }

    pub fn map_to_row<F>(&mut self, col: usize, op: F) -> &Self 
    where
        F: Fn(byte::Byte) -> byte::Byte
    {
        for val in self[col].iter_mut() {
            *val = op(*val)
        }
        self
    }

    pub fn transpose(mut self) -> Self {
        for i in 0..4 {
            for j in i..4 {
                if i != j { 
                    let mem_cpy_item: _ = self[i][j];
                    self[i][j] = std::mem::replace(&mut self[j][i], mem_cpy_item)
                }
            }
        }
        self
    }
}

impl ops::BitXor<Self> for State {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs)
            .map(|(l,r)| l ^ r)
            .collect()
    }
}


impl ops::Index<usize> for State {
    type Output = word::Word;
    
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl ops::IndexMut<usize> for State {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl IntoIterator for State {
    type Item = word::Word;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_vec().into_iter()
    }
}

impl From<[byte::Byte;16]> for State {
    fn from(long_form: [byte::Byte; 16]) -> Self {
        let mut buf: Self = Self::default();
        for (col, chunk) in long_form.chunks(4).enumerate() {
            buf[col].clone_from_slice(chunk)
        }
        buf
    }
}

impl From<[u8;16]> for State {
    fn from(long_form: [u8; 16]) -> Self {
        let mut buf: [byte::Byte; 16] = [byte::Byte::from(0); 16];
        for (idx,byte) in buf.iter_mut().enumerate() {
            *byte = byte::Byte::from(long_form[idx])
        }
        State::from(buf)
    }
}

impl From<[word::Word; 4]> for State {
    fn from(inner: [word::Word; 4]) -> Self {
        State(inner)
    }
}

impl iter::FromIterator<word::Word> for State {
    fn from_iter<I: IntoIterator<Item=word::Word>>(object: I) -> Self {
        let mut iter: _ = object.into_iter();
        let arr: [word::Word; 4] = [
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

impl Default for State {
    fn default() -> Self {
        State::from([ 
            word::Word::from([0;4]),
            word::Word::from([0;4]),
            word::Word::from([0;4]),
            word::Word::from([0;4]),
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl State {
        fn test_vector() -> Self {
            State::from([
                0, 1, 2, 3,
                4, 5, 6, 7,
                8, 9, 10, 11,
                12, 13, 14, 15
            ])
        }
    }

    #[test]
    fn test_from() {
        let test: _ = State::test_vector();
        let exp: _ = State::from([
            word::Word::from([0, 1, 2, 3]),
            word::Word::from([4, 5, 6, 7]),
            word::Word::from([8, 9, 10, 11]),
            word::Word::from([12, 13, 14, 15])
        ]);   
        assert_eq!(test,exp);   
    }

    #[test]
    fn test_iter() {
        let test: _ = State::test_vector();
        let exp: _ = State::from([
            0, 1, 2, 3,
            4, 5, 6, 7,
            8, 9, 10, 11,
            12, 13, 14, 15
        ]);

        println!("{:?}\n{:?}", test, exp);

        for (exp_word, test_word) in exp.into_iter()
            .zip(test.into_iter())
         {
            for (exp_byte, test_byte) in exp_word.into_iter()
                .zip(test_word.into_iter())
            {
                assert_eq!(exp_byte, test_byte)
            }
        }
    }

    #[test]
    fn test_index() {
        let state: _ = State::test_vector();
        assert_eq!(byte::Byte::from(15), state[3][3]);
        assert_eq!(byte::Byte::from(9), state[2][1]);
    }

    #[test]
    fn test_col() {
        let state: _ = State::test_vector();
        let test: _ = &state.col(0);
        let exp: _ = &word::Word::from([0,4,8,12]);
        assert_eq!(test, exp);
    }
    
    #[test]
    fn test_row() {
        let state: _ = State::test_vector();
        let test: _ = state.row(0);
        let exp: _ = word::Word::from([0, 1, 2, 3]);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_map_to_col() {
        let mut state: _ = State::test_vector();
        let exp: _ = word::Word::from([6,14,22,30]);
        state.map_to_col(3, |x| x * 2.into());
        assert_eq!(state.col(3), exp)
    }


    #[test]
    fn test_map_to_row() {
        let mut state: _ = State::test_vector();
        let exp: _ = word::Word::from([24,26,28,30]);
        state.map_to_row(3, |x| x * 2.into());
        assert_eq!(state.row(3), exp)
    }

    #[test]
    fn test_into_cols() {
        let state: _ = State::test_vector();
        let test: _ = state.into_cols().next().unwrap();
        let exp: _ = word::Word::from([0, 4, 8, 12]);
        assert_eq!(test, exp)
    }

    #[test]
    fn test_into_rows() {
        let state: _ = State::test_vector();
        let test: _ = state.into_rows().next().unwrap();
        let exp: _ = word::Word::from([0, 1, 2, 3]);
        assert_eq!(test, exp)
    }
}
