//!
//! the match line, regex text filter like a grep of linux command.
//!
//! ```text
//! Usage:
//!   aki-mline [options]
//!
//! match line, regex text filter like a grep.
//!
//! Options:
//!       --color <when>    use markers to highlight the matching strings
//!   -e, --exp <exp>       regular expression
//!   -s, --str <string>    simple string match
//!   -i, --inverse         output non-matching lines.
//!
//!   -H, --help        display this help and exit
//!   -V, --version     display version information and exit
//!
//! Option Parameters:
//!   <when>    'always', 'never', or 'auto'
//!   <exp>     regular expression
//!   <string>  simple string, non regular expression
//!
//! Environments:
//!   AKI_MLINE_COLOR_SEQ_ST    color start sequence specified by ansi
//!   AKI_MLINE_COLOR_SEQ_ED    color end sequence specified by ansi
//! ```
//!
//! # Examples
//!
//! ### Command line example 1
//!
//! Extract "`arm.*-gnu`" from the rustup target list
//!
//! ```text
//! rustup target list | aki-mline -e "arm.*-gnu"
//! ```
//!
//! result output :
//!
//! ![out rustup image]
//!
//! [out rustup image]: https://raw.githubusercontent.com/aki-akaguma/aki-mline/main/img/out-rustup-1.png
//!
//!
//! ### Command line example 2
//!
//! Extract "`apple`" from the rustup target list
//!
//! ```text
//! rustup target list | aki-mline -s "apple"
//! ```
//!
//! result output :
//!
//! ![out rustup image 2]
//!
//! [out rustup image 2]: https://raw.githubusercontent.com/aki-akaguma/aki-mline/main/img/out-rustup-2.png
//!
//! ### Library example
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute
//!

#[macro_use]
extern crate anyhow;

pub mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::*;
use std::io::Write;

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
///     "mline", &["-e", "Error:.*"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let env = conf::EnvConf::new();
    execute_env(sioe, prog_name, args, &env)
}

pub fn execute_env(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: &[&str],
    env: &conf::EnvConf,
) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf, &env)
}
