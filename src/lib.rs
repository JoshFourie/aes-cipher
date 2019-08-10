#![warn(clippy::all)]

mod boxes;
mod key;
mod text;
mod primitives;

use boxes::{round, schedule};

pub struct AES128 {
    pub plaintext: text::Plain,
    pub key: key::Key,
    internal: AES128Internal
}

struct AES128Internal {
    round: round::RoundFunction,
    schedule: schedule::KeyScheduleFunction
}
