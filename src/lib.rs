#![warn(clippy::all)]

mod boxes;
mod primitives;

use primitives::{state};

#[derive(Debug)]
pub struct RjindaelCipher {
    text: state::State,
    key: state::State,
}

impl RjindaelCipher {
    fn new(text: state::State, key: state::State) -> Self {
        Self { text, key }
    }

    fn encrypt(self) -> state::State {
        let mut ksf: _ = boxes::KeySchedule::new(self.key);
        let mut rnd: _ = boxes::Round::new(self.text);

        for _ in 0..9 {
            let skey: _ = ksf.next();
            rnd.next(skey);
        }
        let last_key: _ = ksf.next();
        let cipher: _ = rnd.last(last_key);

        cipher
    }

    fn decrypt(self) -> state::State {
        let mut rksf: _ = boxes::ReverseKeySchedule::new(self.key);
        let mut rrnd: _ = boxes::ReverseRound::new(self.text);

        for _ in 0..9 {
            let skey: _ = rksf.next();
            rrnd.next(&skey);
        }
        let initial_key: _ = rksf.next();
        let plain: _ = rrnd.last(&initial_key);

        plain
    }
}
