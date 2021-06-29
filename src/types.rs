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
