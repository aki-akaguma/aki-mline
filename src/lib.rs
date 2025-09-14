/*!
the match line, regex text filter like a grep of linux command.

# Features

- the match line, regex text filter like a grep of linux command.
- minimum support rustc 1.65.0 (897e37553 2022-11-02)

# Command help

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

Option Parameters:
  <when>    'always', 'never', or 'auto'
  <exp>     regular expression
  <string>  simple string, non regular expression

Environments:
  AKI_MLINE_COLOR_SEQ_ST    color start sequence specified by ansi
  AKI_MLINE_COLOR_SEQ_ED    color end sequence specified by ansi
```

# Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-mline
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Examples

## Command line example 1

Extract "`arm.*-gnu`" from the rustup target list

```text
rustup target list | aki-mline -e "arm.*-gnu"
```

result output :

![out rustup image]

[out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mline/main/img/out-rustup-1.png


## Command line example 2

Extract "`apple`" from the rustup target list

```text
rustup target list | aki-mline -s "apple"
```

result output :

![out rustup image 2]

[out rustup image 2]: https://raw.githubusercontent.com/aki-akaguma/aki-mline/main/img/out-rustup-2.png

# Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

*/
#[macro_use]
extern crate anyhow;

pub mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::*;

const TRY_HELP_MSG: &str = "Try --help for help.";

///
/// execute mline
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "mline"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_mline::execute(&RunnelIoeBuilder::new().build(),
///     "mline", ["-e", "Error:.*"]);
/// ```
///
pub fn execute<I, S>(sioe: &RunnelIoe, prog_name: &str, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    execute_with_env(sioe, prog_name, args, vec![("", "")])
}

///
/// execute mline with environments
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "mline"
///   - args: parameter arguments.
///   - env: environments array.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// example:
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_mline::execute_with_env(&RunnelIoeBuilder::new().build(),
///     "mline",
///     ["-e", "Error:.*"],
///     vec![
///         ("AKI_MLINE_COLOR_SEQ_ST", "<S>"),
///         ("AKI_MLINE_COLOR_SEQ_ED","<E>"),
///     ]
/// );
/// ```
///
pub fn execute_with_env<I, S, IKV, K, V>(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: I,
    env: IKV,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
    IKV: IntoIterator<Item = (K, V)>,
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let env_cnf: conf::EnvConf = env.into();
    //
    match conf::parse_cmdopts(prog_name, &args_str) {
        Ok(conf) => run::run(sioe, &conf, &env_cnf),
        Err(errs) => {
            if let Some(err) = errs.iter().find(|e| e.is_help() || e.is_version()) {
                sioe.pg_out().write_line(err.to_string())?;
                Ok(())
            } else {
                Err(anyhow!("{errs}\n{TRY_HELP_MSG}"))
            }
        }
    }
}
