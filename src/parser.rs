use crate::lang::Language;

#[derive(Debug)]
pub struct Parser {
  text: String, // iter: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Parser {
  pub fn new(text: String) -> Parser {
    Parser {
      text, // iter: text.chars().peekable(),
    }
  }

  /*
  fn next_char(&mut self) -> Option<&char> {
    self.iter.peek()
  }

  fn consume_char(&mut self) -> Option<char> {
    self.iter.next()
  }

  fn consume_while<F>(&mut self, test_fun: F) -> String
  where
    F: Fn(&char) -> bool,
  {
    let mut result = String::new();
    while !self.next_char().is_none() && test_fun(self.next_char().unwrap_or(&' ')) {
      result.push(self.consume_char().unwrap());
    }
    result
  }

  fn consume_whitespace(&mut self) {
    self.consume_while(|c| c.is_whitespace());
  }

  fn consume_until_newline(&mut self) -> String {
    let res = self.consume_while(|c| c != &'\n' && c != &'\r');
    self.consume_char();
    res
  }

  fn consume_ascii(&mut self) -> String {
    self.consume_while(|c| c.is_ascii())
  }
  */

  pub fn parse(self) -> Result<Vec<Language<'a>>, String> {
    let mut split = self.text.as_str().lines();

    // let mut reading_lang = false;
    let mut langs = Vec::<Language>::new();
    let mut lang = Vec::<&str>::new();

    loop {
      let line = split.next();
      if line.is_none() || line.unwrap_or_default().starts_with('-') {
        if !lang.is_empty() {
          langs.push(Language::parse(&lang.as_slice())?);
        }
        lang = Vec::new();
        // reading_lang = false;
        if line.is_none() {
          break;
        } else {
          continue;
        }
      }
      let line = line.unwrap().trim();
      if !line.is_empty() && !line.starts_with('#') {
        lang.push(line)
      }
    }

    Ok(langs)
  }
}
