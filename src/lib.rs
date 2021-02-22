//!
//! the match line, regex text filter program like grep.
//!
//! ```text
//! Usage:
//!   aki-mline [options]
//!
//! match line, regex text filter, like grep.
//!
//!       --color <when>   use markers to highlight the matching strings
//!   -e, --exp <exp>      regular expression
//!   -v, --invert-match   select non-matching lines
//!
//!   -H, --help     display this help and exit
//!   -V, --version  display version information and exit
//!
//! Env:
//!   RUST_GREP_COLOR_ST   color start sequence
//!   RUST_GREP_COLOR_ED   color end sequence
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
//! ### Library example
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute
//!

#[macro_use]
extern crate anyhow;

mod conf;
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
///   - program: program name. etc. "gsub"
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
    run::run(sioe, &conf)
}
