use crate::primitives::{sbox, state, byte, word};

pub struct RoundFunction {
    sbox: sbox::SubBox,
    skey: state::State,
    state: state::State
}

impl RoundFunction {
    pub fn new(skey: state::State, state: state::State) -> Self {
        Self {
            skey, 
            state,
            sbox: sbox::SubBox::default()
        }
    }

    pub fn round_function(self) -> Self {
        self.sub_and_shift()
            .mix()
            .xor_with_key()
    }

    fn sub_and_shift(mut self) -> Self {
        self.state = self.swap_state()
            .into_rows()
            .enumerate()
            .map(|(idx, row)| row.substitute(&self.sbox) << idx)
            .collect();
        self
    }

    fn mix(mut self) -> Self {
        let perm: _ = state::State::from([
            0x02, 0x03, 0x01, 0x01,
            0x01, 0x02, 0x03, 0x01,
            0x01, 0x01, 0x02, 0x03,
            0x03, 0x01, 0x01, 0x02
        ]);
        self.state = {
            let mut buf: _ = state::State::default();
            for i in 0..4 {
                for j in 0..4 {
                    let mut sigma: _ = byte::Byte::zero();
                    for k in 0..4 {
                        sigma = sigma + (perm[i][k] * self.state[k][j])
                    }
                    buf[i][j] = sigma
                }
            }  
            buf
        };
        self
    }

    fn xor_with_key(mut self) -> Self {
        self.state = self.state ^ self.skey.clone();
        self
    }

    fn swap_state(&mut self) -> state::State {
        std::mem::replace(&mut self.state, state::State::default())
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_columns() {
        // let initial: _ = state::State::from([
        //     0xab, 0x8b, 0x89, 0x35,
        //     0x40, 0x7f, 0xf1, 0x05,
        //     0xf0, 0xfc, 0x18, 0x3f,
        //     0xc4, 0xe4, 0x4e, 0x2f
        // ]);
        // let rf: _ = RoundFunction::new(state::State::default(), initial);
        // let test: _ = rf.mix();
        // let exp: _ = state::State::from([
        //     0xb9, 0x94, 0x57, 0x75,
        //     0xe4, 0x8e, 0x16, 0x51,
        //     0x47, 0x20, 0x9a, 0x3f,
        //     0xc5, 0xd6, 0xf5, 0x3b
        // ]);

        let initial: _ = state::State::from([
            0x87, 0xf2, 0x4d, 0x97,
            0x6e, 0x4c, 0x90, 0xec,
            0x46, 0xe7, 0x4a, 0xc3,
            0xa6, 0x8c, 0xd8, 0x95
        ]);
        let rf: _ = RoundFunction::new(state::State::default(), initial);
        let test: _ = rf.mix();
        let exp: _ = state::State::from([
            0x47, 0x40, 0xa3, 0x4c,
            0x37, 0xd4, 0x70, 0x9f,
            0x94, 0xe4, 0x3a, 0x42,
            0xed, 0xa5, 0xa6, 0xbc
        ]);


        assert_eq!(test.state, exp)
    }

    #[test]
    fn test_sub_and_shift() {
        let initial: _ = state::State::from([
            0x0e, 0xce, 0xf2, 0xd9,
            0x36, 0x72, 0x6b, 0x2b,
            0x34, 0x25, 0x17, 0x55,
            0xae, 0xb6, 0x4e, 0x88
        ]);
        let rf: _ = RoundFunction::new(state::State::default(), initial);
        let test: _ = rf.sub_and_shift();
        let exp: _ = state::State::from([
            0xab, 0x8b, 0x89, 0x35,
            0x40, 0x7f, 0xf1, 0x05,
            0xf0, 0xfc, 0x18, 0x3f,
            0xc4, 0xe4, 0x4e, 0x2f
        ]);

        assert_eq!(test.state, exp)
    }
}
