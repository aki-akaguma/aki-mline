# aki-mline

the match line, regex text filter like a grep of linux command.

## Features

- the match line, regex text filter like a grep of linux command.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Command help

```
aki-mline --help
```

```
Usage:
  aki-mline [options]

match line, regex text filter like a grep.

Options:
      --color <when>    use markers to highlight the matching strings
  -e, --exp <exp>       regular expression
  -s, --str <string>    simple string match
  -i, --inverse         output non-matching lines.

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Option Parameters:
  <when>    'always', 'never', or 'auto'
  <exp>     regular expression
  <string>  simple string, non regular expression

Environments:
  AKI_MLINE_COLOR_SEQ_ST    color start sequence specified by ansi
  AKI_MLINE_COLOR_SEQ_ED    color end sequence specified by ansi
```

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-mline
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

### Command line example 1

Extract "`arm.*-gnu`" from the rustup target list

```
rustup target list | aki-mline -e "arm.*-gnu"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mline/main/img/out-rustup-1.png


### Command line example 2

Extract "`apple`" from the rustup target list

```
rustup target list | aki-mline -s "apple"
```

result output :

![out rustup image 2]

[out rustup image 2]: https://raw.githubusercontent.com/aki-akaguma/aki-mline/main/img/out-rustup-2.png

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-mline/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
