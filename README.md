# Theme parser for `lsfp`

**This README file is outdated, take a look at the formal specification in [SPEC.md](https://github.com/HipyCas/theme-parser/blob/master/SPEC.md) instead. I am currently working on updating this README file, but the formal specification will always have precedence over this README.**

This repository contains a simple theme parser for the command line tool [`lsfp`](https://github.com/The-Noah/lsfp). This code is located in this repository for testing and easier development, but after it is successfully tested, the code will be moved into the main repository.

## The theme file syntax

A theme file consists of multiple sections, representing languages and/or extensions that should be styled with the same color or icon. This sections are delimited by a line starting with a dash (`-`), taking the next lines until a new dash. This dash **MUST** be the first character on the line, but as the line is only used as an indicator of a new section start, everything after that dash is ignored, meaning that you can write as many dashes as you may want or write the language name or some comments on there.

Inside each section, there are different key-value pairs, being the _extensions_ key the most important one, as it is the one that the program will use to recognize files and styles associated. As of today, there are only three keys available, which also depend on the features that you compiled `lsfp` with: extensions (primary key which works across al features), color (an RGB color for the extensions, only available under _color_ feature) and icon (an hexadecimal value made up of 4 characters, which can be found at [NerdFont's cheat-sheet](https://www.nerdfonts.com/cheat-sheet), only available under _icons_ feature). Keys and values are written as `<key>=<value>`, and any other syntax will no match. All empty lines are ignored, and a key without a value or line that does not match the `<key>=<value>` pattern will throw an error, while an unrecognized key will not be of any problem.

Here is an example theme file:

```
--- JS (mjs for modules with node). TODO Add more
extensions=js,mjs
color=255,220,0
icon=f898

--- TS Maybe there are more extensions? Revise icon
extensions=ts
color=0,31,63

icon=e628

-- IMPORTANT Rust, don't erase this by any means!
color=255,65,54
extensions=rs
icon=e7a8
```

A more detailed specification is on its way. When done, it will temporally live in [SPEC.md](https://github.com/HipyCas/theme-parser/blob/master/SPEC.md), until it gets moved to the mainstream repository along with the code.

You can also find a test theme file in the root of the repository, called [test.theme](https://github.com/HipyCas/theme-parser/blob/master/test.theme) (extension is only for easy identification). This theme file is read by the main script for testing purposes, and you may want to try modify it and test it.

## License

Both the code of the theme parser and the specification are distributed under the same license as `lsfp`, as it is part of the program. This license is the MIT license, which allows you to modify and use this parser and specification for both personal and commercial projects, either open source or closed source. See the full LICENSE file at [The-Noah/lsfp](https://github.com/The-Noah/lsfp/blob/master/LICENSE)

## TODO

- ~~Allow extra non empty lines as comments, maybe make them start with a `#` or similar~~

- Allow to set default icon and colors, same as for directories (collapsed and expanded)

- Allow for `:` instead of `=` separator

- ~~**IMPORTANT** Formal specification file~~

- Make `extensions` key required

- Get a better heading than _Pairs_ for SPEC.md <- starting to like _Pairs_

- ~~Better error messages, maybe by indicating the line of the error (look at https://llogiq.github.io/2017/06/01/perf-pitfalls.html), use `for (i, line) in text.lines().enumerate() { /* ... */ }`~~

- ~~Return `struct ParseError { line: usize, text: String, error_msg: String }` for Result.Err~~

- Use `;` to separate lines instead of line break

- Make `ParserError.line` of `Line` type and drop `ParserError.text` attribute

- Make `Line` accept `AsRef<str>` instead of directly `&str` so you can use `String` in `ParserError`

- Return result in `Vec<u8>.as_color()` instead of `(0,0,0)`

- Improve error message ("invalid digit found in string") in lang parser
