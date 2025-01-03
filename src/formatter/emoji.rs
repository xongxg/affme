use crate::random::pick;

#[derive(Debug)]
pub struct Emoji<'a> {
    emojis: [&'a str; 7],
}

impl<'a> Emoji<'a> {
    pub fn new() -> Self {
        let emojis = ["😍", "😎", "🧸", "😉", "👍", "💪", "✨"];
        Emoji { emojis }
    }

    pub fn random(&self) -> &'a str {
        pick(&self.emojis)
    }
}
