# `lsfp` Theme file specification

This is the formal specification for `lsfp`'s theme file grammar. `lsfp` will only be able to parse files that strictly follow this specification, falling back to the default theme when the file is not of the correct format.

## Sections

A theme file is organized into multiple sections, identified by the `extensions` associated with it. Section start is delimited by a dash (`-`) at the beginning of the line, being everything after the first dash discarded (`lsfp` will not analyze any text after it, you can write anything after the first dahs), and ends with the next line starting by a dash (the next section start delimiter).

Each section contains `key=value` pairs, which are explained in the [Pairs](#Pairs) section. A complete section could be the following (see [Example](#Example) for multiple sections):

```
--- JS (mjs for modules with node). TODO Add more
    extensions=js,mjs
    color=255,220,0
    icon=f898
```

## Pairs

The theme styles are indicated in form of `key=value` pairs inside each section, being `extensions` the primary key (the one that is used for identification by `lsfp`), and the other dependant of `lsfp`'s features. There are three available pairs are the following as of now:

- `extensions`: An array-like of extensions, which should be styled with the styles specified in the section. If this is not specified, the styles in the section will not be applied to any file. The syntax for this key is as follows: `color={ext1},{ext2},{..exts}`, as an example `extensions=js,mjs`.

- `color`: A tuple-like color, indicating the color with which the extensions should be printed. It must be a tuple of 3 numbers, in the range of 0 to 255 included (rust's `u8` type). This configuration is only available if `lsfp` is compiled with the _color_ feature (see [Features](#Features) for more about features). The syntax for this key is as follows: `color={r},{g},{b}`, as an example `color=255,220,0`.

- `icon`: A four hexadecimal character string representing the icon Unicode code. Icons are supposed to be used with NerdFonts (icons used by default in `lsfp` depend on this font), and you can find a full list of icons and their hexadecimal string on the [NerdFont cheat-sheet](https://www.nerdfonts.com/cheat-sheet); but you may use any form of icon support if it is based in 4 character hexadecimal Unicode value. This configuration is only available if `lsfp` is compiled with the _icons_ feature (see [Features](#Features) for more about features). The syntax for this key is as follows: `icon={4 chars}`, as an example `icon=f898`.

## Comments

You can write comments by using a hashtag (`#`), wither at the beginning of a line or at the end of a pair. Any text in the comment is completely ignored by the parser. For example, the following are both valid comments:

```txt
icon=f2a4 # Beautiful solid icon
# TODO Add color, get some ideas online
```

## Features

Themes in `lsfp` are enabled under the feature _themes_, and therefore if the feature is not enabled for the build, no theme will be parsed/loaded in any case.

For the `colors` and `icon` key the features _color_ and _icons_ (respectively) must be enabled for the build. In case of those not being present, the parser will still give a valid language style, yet the color or icon will be empty and will not be printed by `lsfp` anyway.

In GitHub releases, all builds include all features except `lsfp-lite.exe` (Windows), which includes none. This means that all come with theme support builtin and support for all available settings.

## Flexibilities

The parser presents some flexibilities in how you may write values, which are outlined below.

- Leading and trailing commas are ignored in both `color` and `extensions` settings, meaning that both `color=,9,0,12` and `extensions=c,h,o,` are both valid pairs.

- Spaces at beginning and end of lines, keys and values are removed, therefore both `<tab|spaces># comment` and `<spaces>icon = f56a <spaces>` are valid. **LEADING SPACES ARE NOT ALLOWED IN SECTION BEGINNINGS, DASH MUST ALWAYS BE THE FIRST CHARACTER**

## Example

Below is present a full example (a real theme file would be much bigger):

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

## Distribution

Theme files are easily distributable. Due to the fact that settings that require a feature to be enabled are ignored by the parser if the feature is not enabled, someone with the _color_ feature and someone without it could be using the same theme file.

`lsfp` provides official themes which you can find in the _themes_ folder and distributed as a zip file on GitHub releases next to the built binaries.

## License

Both the code of the theme parser and the specification are distributed under the same license as `lsfp`, as it is part of the program. This license is the MIT license, which allows you to modify and use this parser and specification for both personal and commercial projects, either open source or closed source. See the full LICENSE file at [The-Noah/lsfp](https://github.com/The-Noah/lsfp/blob/master/LICENSE)
