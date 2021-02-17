# aki-mline

*aki-mline* is the regex text filter program like grep.

## Features

*aki-mline*  is match line. this is filtering text line by regex, like grep.

* command help

```text
aki-mline --help
```

```text
Usage:
  aki-mline [options]

match line, regex text filter, like grep.

      --color <when>   use markers to highlight the matching strings
  -e, --exp <exp>      regular expression
  -v, --invert-match   select non-matching lines

  -H, --help     display this help and exit
  -V, --version  display version information and exit

Env:
  RUST_GREP_COLOR_ST   color start sequence
  RUST_GREP_COLOR_ED   color end sequence
```

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-mline
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

#### Command line example 1

Extract "`arm.*-gnu`" from the rustup target list

```
rustup target list | aki-mline -e "arm.*-gnu"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mline/main/img/out-rustup-1.png

#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
