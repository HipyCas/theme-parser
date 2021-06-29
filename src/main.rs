// mod _original;
mod lang;
mod parser;
mod types;

pub use types::{Color, VecConvert};

fn main() {
    println!(
        "{:?}",
        &(parser::Parser::new(include_str!("../test.theme").to_owned())
            .parse()
            .expect("Error while parsing theme")
            .iter_mut()
            .map(|l| l.as_style())
            .collect::<Vec<_>>())[..] as types::Theme
    )
}
