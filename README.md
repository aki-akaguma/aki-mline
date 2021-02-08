# aki-mline

*aki-mline* is the regex text filter program.

## Features

*aki-mline*  is match line. this is filtering text line by regex, like grep.

* example

command:
```
`aki-mline` -H
```

* minimum support rustc 1.38.0

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

```
cat text-file | aki-mline -e "^Error" -e "^Warn"
```
