//!
//! the substitude text program.
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
/// use runnel::medium::stdioe::{StreamInStdin,StreamOutStdout,StreamErrStderr};
/// use runnel::StreamIoe;
///
/// let r = libaki_mline::execute(&StreamIoe{
///     sin: Box::new(StreamInStdin::default()),
///     sout: Box::new(StreamOutStdout::default()),
///     serr: Box::new(StreamErrStderr::default()),
/// }, "mcolor", &["-r", "Error", "-g", "Warn"]);
/// ```
///
pub fn execute(sioe: &StreamIoe, program: &str, args: &[&str]) -> anyhow::Result<()> {
    //
    let conf = match conf::parse_cmdopts(program, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.sout.lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
