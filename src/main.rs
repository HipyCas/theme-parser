// mod _original;
mod home;
mod lang;
mod parser;
mod types;

pub use types::{Color, Line, ParserError, VecConvert};

fn main() {
    // println!("{:?}", home::home_dir());
    let theme = get_theme(std::env::args().nth(1));
    println!(
        "{:?}",
        parser::Parser::new(theme)
            .parse()
            .expect("Error while parsing theme")
            .iter_mut()
            .map(|l| l.as_style())
            .collect::<Vec<_>>()
            .as_slice() as types::Theme
    )
}

fn get_theme(name: Option<String>) -> String {
    match name {
        Some(path) => {
            if std::path::Path::new(&path).exists() {
                std::fs::read_to_string(&path).unwrap()
            } else {
                let mut text = String::new();
                if !std::path::Path::new(std::path::Path::new(
                    &home::home_dir().unwrap().join(".lsfp-themes"),
                ))
                .exists()
                {
                    std::fs::create_dir(std::path::Path::new(
                        &home::home_dir().unwrap().join(".lsfp-themes"),
                    ))
                    .unwrap();
                };
                for item in std::fs::read_dir(std::path::Path::new(
                    &home::home_dir().unwrap().join(".lsfp-themes"),
                ))
                .unwrap()
                {
                    if item.unwrap().file_name() == std::ffi::OsString::from(&path) {
                        text = std::fs::read_to_string(
                            home::home_dir().unwrap().join(".lsfp-themes").join(&path),
                        )
                        .unwrap()
                    }
                }
                text
            }
        }
        None => include_str!("../test.theme").to_owned(),
    }
}
