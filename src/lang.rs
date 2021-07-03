use crate::types::FileStyle;
use crate::{Color, VecConvert};

#[derive(Debug)]
pub struct Language<'a> {
  extensions: Vec<String>,
  extensions_str: Vec<&'a str>,
  color: Color,
  icon: String,
}

impl<'a> Language<'a> {
  pub fn new(extensions: Vec<String>, color: Color, icon: String) -> Self {
    Self {
      extensions,
      extensions_str: Vec::new(),
      color,
      icon,
    }
  }

  pub fn empty() -> Self {
    Self {
      extensions: Vec::new(),
      extensions_str: Vec::new(),
      color: (0, 0, 0),
      icon: "".to_owned(),
    }
  }

  // pub fn parse<P>(text: std::iter::TakeWhile<String, P>) -> Language
  // where
  //   P: FnMut(&String) -> bool,
  // {

  //   Language::empty()
  // }

  pub fn parse(text: &Vec<&str>) -> Result<Language<'a>, String> {
    println!("{:?}", text);

    let mut extensions = Vec::new();
    let mut color: Vec<u8> = Vec::new();
    let mut icon: String = String::new();

    for (key, value) in text
      .into_iter()
      .map(|pair| pair.split_once('=').unwrap_or(("", "")))
      .map(|(key, value)| (key.trim(), value.trim()))
    {
      if key.is_empty() && value.is_empty() {
        return Err(
          format!("Line must include a pair of key and value separated by a equal (=) in text line \"{}={}\"", key, value),
        );
      } else if key.is_empty() {
        return Err(format!("Missing key in text line \"{}={}\"", key, value));
      } else if value.is_empty() {
        return Err(format!("Missing value in text line \"{}={}\"", key, value));
      } else if key == "e" || key == "extensions" {
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
      } else if key == "c" || key == "color" {
        println!("Key color");
        #[cfg(feature = "color")]
        {
          let mut num = String::new();
          for (i, mut ch) in value.char_indices() {
            println!(">>>>> Parsing character from {}", ch);
            if color.len() >= 3 {
              return Err(
                "Only 3 numbers (red, green, blue) should be provided for color".to_owned(),
              );
            }
            if num.len() >= 3 {
              return Err("Color must range from 0 to 255".to_owned());
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
        }
      } else if key == "i" || key == "icon" {
        println!("> Parsing icon ({}={})", key, value);
        #[cfg(feature = "icons")]
        {
          println!(">> Got into config");
          if value.len() > 4 {
            return Err(format!("Icon must be 4 characters, 2 hexadecimal values, but found \"{}\", which is {} characters long", value, value.len()));
          }
          if !value.chars().all(|c| c.is_digit(16)) {
            return Err(format!(
              "One or more characters in \"{}\" icon are not valid hexadecimal characters",
              value
            ));
          }
          icon = value.to_owned()
        }
      }
    }

    Ok(Language::new(extensions, color.as_color(), icon))
  }

  pub fn as_style(&'a mut self) -> FileStyle {
    self.extensions_str = self.extensions.iter().map(String::as_str).collect();
    // self.extensions_str = self.extensions.as_slice();
    (&self.extensions_str[..], self.color, &self.icon)
  }
}
