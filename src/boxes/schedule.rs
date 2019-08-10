use crate::primitives::{sbox,state,byte,word};

pub struct KeySchedule {
    key: state::State,
    sbox: sbox::SubBox,
    rcon: RCON
}

impl KeySchedule {

    const TMP_COL: usize = 3;

    pub fn new(key: state::State) -> Self {
        KeySchedule {
            key,
            sbox: sbox::SubBox::default(),
            rcon: RCON::default()
        }
    }

    pub fn next(&mut self) -> &state::State {
        let tmp: word::Word = self.tmp();
        self.key = self.chain_xor(tmp);
        &self.key
    }

    fn tmp(&mut self) -> word::Word {
        let mut tmp: word::Word = self.key
            .col(Self::TMP_COL)
            .substitute(&self.sbox)
            .rotate();
        tmp[0] = tmp[0] ^ self.rcon.as_inner();
        self.rcon.update();
        tmp
    }

    fn chain_xor(&mut self, tmp: word::Word) -> state::State {
        let mut buf: state::State = Default::default();
        let first_subkey: _ = std::mem::replace(&mut self.key[0], word::Word::default());
        buf[0] = tmp ^ first_subkey;

        for idx in 1..4 {
            let prev_subkey: _ = std::mem::replace(&mut self.key[idx], word::Word::default());
            buf[idx] = buf[idx-1].clone() ^ prev_subkey
        }

        buf
    }
}

struct RCON(byte::Byte);

impl RCON {
    fn update(&mut self) -> &Self {
        let RCON(inner) = self;
        *inner = *inner * byte::Byte::from(2);
        self
    }

    fn as_inner(&self) -> byte::Byte {
        self.0
    }
}

impl Default for RCON {
    fn default() -> Self {
        RCON(byte::Byte::from(1))   
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn test_key_schedule() {
        unimplemented!()
    }

    #[test]
    fn test_rcon_update() {
        let mut rcon: _ = RCON::default();
        rcon.update();
        rcon.update();
        rcon.update();

        assert_eq!(byte::Byte::from(8_u8), rcon.as_inner())
    }

    #[test]
    fn test_rcon_xor() {
        let rcon: _ = RCON::default();
        let mut test: _ = word::Word::from([0xd2, 0x85, 0x46, 0x79]);
        test[0] = test[0] ^ rcon.as_inner();

        let exp: _ = word::Word::from([0xd3, 0x85, 0x46, 0x79]);
        assert_eq!(exp, test)
    }

    #[test]
    fn test_chain_xor() {
        // starting on second round.
        let mut ksf: KeySchedule = {
            let w4: _ = word::Word::from([0xdc, 0x90, 0x37, 0xb0]);
            let w5: _ = word::Word::from([0x9b, 0x49, 0xdf, 0xe9]);
            let w6: _ = word::Word::from([0x97, 0xfe, 0x72, 0x3f]);
            let w7: _ = word::Word::from([0x38, 0x81, 0x15, 0xa7]);
            let key: _ = state::State::from([w4,w5,w6,w7]);
            let mut ksf: _ = KeySchedule::new(key);
            ksf.rcon.update();
            ksf
        };

        let tmp: _ = word::Word::from([0x0e, 0x59, 0x5c, 0x07]);
        let test: _ = ksf.chain_xor(tmp);

        let exp: state::State = {
            let w8: _ = word::Word::from([0xd2, 0xc9, 0x6b, 0xb7]);
            let w9: _ = word::Word::from([0x49, 0x80, 0xb4, 0x5e]);
            let w10: _ = word::Word::from([0xde, 0x7e, 0xc6, 0x61]);
            let w11: _ = word::Word::from([0xe6, 0xff, 0xd3, 0xc6]);
            let key: _ = state::State::from([w8,w9,w10,w11]);
            key
        };

        assert_eq!(exp, test)
    }
}
