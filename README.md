# aki-mline

*aki-mline* is the match line, regex text filter like a grep of linux command.

## Features

*aki-mline*  is the match line, regex text filter like a grep of linux command.

* command help

```text
aki-mline --help
```

```text
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

Env:
  AKI_MLINE_COLOR_ST   color start sequence
  AKI_MLINE_COLOR_ED   color end sequence
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


#### Command line example 2

Extract "`apple`" from the rustup target list

```
rustup target list | aki-mline -s "apple"
```

result output :

![out rustup image 2]

[out rustup image 2]: https://raw.githubusercontent.com/aki-akaguma/aki-mline/main/img/out-rustup-2.png

#### Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
