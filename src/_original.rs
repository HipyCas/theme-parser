use crate::constants::{FileExtensionStyle, Theme};

pub fn get_theme<'a>(name: &str, parser: &'a mut Parser<'a>) -> Option<Theme<'a, String>> {
  // let mut parser = Parser::new(include_str!("test_theme.txt"));
  let theme = parser.parse().unwrap();
  match name {
    // "" | "default" => Some(crate::constants::FILE_EXTENSION_COLORS),
    _ => Some(theme),
  }
}

pub struct Parser<'a> {
  iter: std::iter::Peekable<std::str::Chars<'a>>,
  theme: Vec<FileExtensionStyle<'a, String>>,
  total_extensions: Vec<Vec<String>>,
  total_colors: Vec<(u8, u8, u8)>,
  total_icons: Vec<String>,
}

impl<'a> Parser<'a> {
  pub fn new(text: &'a str) -> Parser<'a> {
    Parser {
      iter: text.chars().peekable(),
      theme: Vec::new(),
      total_extensions: Vec::new(),
      total_colors: Vec::new(),
      total_icons: Vec::new(),
    }
  }

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

  fn parse(&mut self) -> Result<Theme<String>, String> {
    println!("to parse: {:?}", self.iter);
    let mut theme: Vec<FileExtensionStyle<String>> = Vec::new(); // To store the themes
    let mut total_extensions: Vec<Vec<String>> = Vec::new();
    // let mut total_colors: Vec<(u8, u8, u8)> = Vec::new();
    // let mut total_icons: Vec<String> = Vec::new();

    while self.next_char().is_some() {
      println!("> Iterating: {:?}", self.next_char());
      let mut extensions: Vec<String> = Vec::new(); // Place to store extensions
      #[cfg(feature = "color")]
      let mut color: Vec<u8> = Vec::new(); // Place to store color (r, g, b)
      #[cfg(feature = "icons")]
      let mut icon: String = String::new(); // Place to store icon

      if self.next_char().unwrap() != &'-' {
        println!(">> {:?} is not a section start", self.next_char());
        return Err("Expected file/section to begin with one or more dashes (-)".to_owned());
      }
      println!(">> Consumed whole line: {}", self.consume_until_newline());
      while self.next_char().unwrap_or(&'-') != &'-' {
        println!(
          ">>> Parsing args, as next char is not '-': {:?}",
          self.next_char()
        );
        // TODO Consume whitespaces between key, =, and value
        let key = self.consume_while(|c| c != &'=' && c != &':' && !c.is_whitespace()); // Consume key
        self.consume_char(); // Consume and discard equal or ':'
        let value = self.consume_until_newline(); // Consume value

        println!(">>> Obtained key {} and value {}", key, value);

        if key == "extensions" || key == "e" {
          // Parse extensions
          println!(">>>> Parsing extensions from {}", value);
          let mut ext = String::new();
          for (i, mut ch) in value.char_indices() {
            println!(">>>>> Parsing character {}", ch);
            if i == value.len() - 1 {
              ext.push(ch);
              ch = ',';
            }
            if ch != ',' {
              ext.push(ch)
            } else {
              extensions.push(ext);
              ext = String::new();
            }
          }
        } else if key == "color" || key == "c" {
          println!(">>>> Parsing color from {}", value);
          #[cfg(feature = "color")]
          let mut num = String::new();
          #[cfg(feature = "color")]
          for (i, mut ch) in value.char_indices() {
            println!(">>>>> Parsing character from {}", ch);
            if color.len() >= 3 {
              return Err(
                "Only 3 numbers (red, green, blue) should be provided for color".to_owned(),
              );
            }
            if i == value.len() - 1 {
              num.push(ch);
              ch = ',';
            }
            if ch != ',' {
              num.push(ch)
            } else {
              color.push(match num.parse() {
                Ok(num) => num,
                Err(err) => return Err(err.to_string()),
              });
              num = String::new();
            }
          }
          println!(">>>> Parsed color: {:?}", color);
        } else if key == "icon" || key == "i" {
          #[cfg(feature = "icons")]
          for ch in value.chars() {
            if icon.len() >= 4 {
              return Err("Icon should only be 4 characters long".to_owned());
            }
            icon.push(ch)
          }
        }
      }
      // let extensions_str: Vec<&str> = extensions.into_iter().map(move |s: String| &'static s).collect();
      // theme.push(&[extensions.tuple()])
      // println!("&[&[{:?}], ({:?}), {}]", extensions, color, icon);
      // theme.push((extensions[..], (color[0], color[1], color[2]), &icon));
      total_extensions.push(extensions);
      self.total_colors.push((color[0], color[1], color[2]));
      self.total_icons.push(icon);
    }
    // // Ok(&[ex])
    // for (((_, extension), color), icon) in Vec::<u8>::new()
    //   .iter()
    //   .zip(total_extensions)
    //   .zip(total_colors)
    //   .zip(total_icons)
    //   .collect::<Vec<(((&_, std::vec::Vec<std::string::String>), (u8, u8, u8)), std::string::String)>>()
    // {
    //   theme.push((&extension[..], color, icon));
    // }

    // Ok(&theme[..])
    self.theme = theme;
    self.total_extensions = total_extensions;
    // Ok(self.theme)

    for (((_, extension), color), icon) in Vec::<u8>::new()
      .iter()
      .zip(&self.total_extensions)
      .zip(&self.total_colors)
      .zip(&self.total_icons)
      .collect::<Vec<(
        ((&u8, &std::vec::Vec<std::string::String>), &(u8, u8, u8)),
        &std::string::String,
      )>>()
    {
      self.theme.push((
        &extension[..],
        (color.0, color.1, color.2),
        icon.to_string(),
      ));
    }

    Ok(&self.theme[..])
  }
}
