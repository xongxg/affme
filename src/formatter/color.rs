use crate::random::pick;

#[derive(Debug)]
pub struct Color<'a> {
    colors: [&'a str; 6],
}

impl<'a> Color<'a> {
    pub fn new() -> Self {
        let colors = ["red", "blue", "green", "yellow", "magenta", "cyan"];
        Color { colors }
    }

    pub fn random(&self) -> &'a str {
        pick(&self.colors)
    }
}
