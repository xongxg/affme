mod affirmation;
mod formatter;
mod random;

use crate::affirmation::Affirmation;
use crate::formatter::format;

pub fn affirm(name: &str) -> String {
    let affirmation = Affirmation::new().random();
    format(affirmation, name)
}
