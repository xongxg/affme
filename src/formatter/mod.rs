use crate::formatter::color::Color;
use crate::formatter::emoji::Emoji;
use colored::Colorize;

mod color;
mod emoji;

pub fn format(affirmation: &str, name: &str) -> String {
    let emoji = Emoji::new();
    let color = Color::new();

    let phrase = format!("{}, {} {}", affirmation, name, emoji.random())
        .color(color.random())
        .bold()
        .to_string();

    format!(
        "{}\n{}\n{}\n{}\n{}",
        "*".repeat(phrase.len() + 2).magenta(),
        format!("*{}*", " ".repeat(phrase.len())).magenta(),
        format!("    ✏️  ...{}  ", phrase,),
        format!("*{}*", " ".repeat(phrase.len())).magenta(),
        "*".repeat(phrase.len() + 2).magenta()
    )
}
