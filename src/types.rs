use std::{error, fmt};

pub type Color = (u8, u8, u8);

pub type FileStyle<'a> = (&'a [&'a str], Color, &'a str);

pub type Theme<'a> = &'a [FileStyle<'a>];

pub trait VecConvert {
  fn as_color(&self) -> Color; // TODO Return results
}

impl VecConvert for Vec<u8> {
  fn as_color(&self) -> Color {
    (
      *self.get(0).unwrap_or(&0),
      *self.get(1).unwrap_or(&0),
      *self.get(2).unwrap_or(&0),
    )
  }
}

pub type Line<'a> = (usize, &'a str);

#[derive(Debug)]
pub struct ParserError {
  line: usize, // TODO Make this Line type and drop text attribute
  text: String,
  msg: String,
}

impl ParserError {
  pub fn new(line: usize, text: String, msg: String) -> ParserError {
    ParserError { line, text, msg }
  }
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&format!(
      "parser error on line {} with contents {}: {}",
      self.line, self.text, self.msg
    ))
  }
}

impl error::Error for ParserError {}
